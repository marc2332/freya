use std::io::{
    Read,
    Write,
};

use portable_pty::{
    CommandBuilder,
    MasterPty,
    PtySize,
    native_pty_system,
};

use crate::{
    backends::{
        TerminalBackend,
        TerminalOutput,
    },
    handle::TerminalError,
};

/// Runs a command in a pseudo-terminal. Nothing is spawned until the backend is handed to
/// [`TerminalHandle::new`](crate::handle::TerminalHandle::new), and dropping it closes the
/// pseudo-terminal, which ends the child process.
pub struct PtyBackend {
    command: CommandBuilder,
    master: Option<Box<dyn MasterPty + Send>>,
    writer: Option<Box<dyn Write + Send>>,
}

impl PtyBackend {
    pub fn new(command: CommandBuilder) -> Self {
        Self {
            command,
            master: None,
            writer: None,
        }
    }
}

impl TerminalBackend for PtyBackend {
    fn start(&mut self, output: TerminalOutput) -> Result<(), TerminalError> {
        let pair = native_pty_system()
            .openpty(PtySize::default())
            .map_err(|e| TerminalError::StartError(e.to_string()))?;
        let writer = pair
            .master
            .take_writer()
            .map_err(|e| TerminalError::StartError(e.to_string()))?;
        pair.slave
            .spawn_command(self.command.clone())
            .map_err(|e| TerminalError::StartError(e.to_string()))?;
        let mut reader = pair
            .master
            .try_clone_reader()
            .map_err(|e| TerminalError::StartError(e.to_string()))?;
        self.writer = Some(writer);
        self.master = Some(pair.master);

        std::thread::spawn(move || {
            let mut buffer = [0u8; 128 * 1024];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) | Err(_) => break,
                    Ok(read) => {
                        if output.write(&buffer[..read]).is_err() {
                            break;
                        }
                    }
                }
            }
            output.close();
        });

        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> Result<(), TerminalError> {
        let writer = self.writer.as_mut().ok_or(TerminalError::Closed)?;
        writer.write_all(data)?;
        writer.flush()?;
        Ok(())
    }

    fn resize(&mut self, rows: u16, cols: u16) {
        if let Some(master) = &self.master {
            let _ = master.resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            });
        }
    }
}
