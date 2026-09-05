#[cfg(feature = "pty")]
pub mod pty;

use futures_channel::mpsc::UnboundedSender;

use crate::handle::TerminalError;

/// Program a [`TerminalHandle`](crate::handle::TerminalHandle) talks to, such as the built-in
/// [`PtyBackend`](crate::backends::pty::PtyBackend) or any other.
/// Dropping the backend must stop whatever it is running.
pub trait TerminalBackend {
    /// Called once when the handle is created.
    fn start(&mut self, output: TerminalOutput) -> Result<(), TerminalError>;

    fn write(&mut self, data: &[u8]) -> Result<(), TerminalError>;

    /// Resize the backend to `rows` and `cols`.
    fn resize(&mut self, rows: u16, cols: u16);
}

/// Sends program output to the terminal grid. The terminal closes once every clone is dropped.
#[derive(Clone)]
pub struct TerminalOutput {
    pub(crate) sender: UnboundedSender<Vec<u8>>,
}

impl TerminalOutput {
    /// Feed bytes to the terminal parser.
    pub fn write(&self, data: impl Into<Vec<u8>>) -> Result<(), TerminalError> {
        self.sender
            .unbounded_send(data.into())
            .map_err(|_| TerminalError::Closed)
    }
}
