use std::{
    fs,
    io::{
        self,
        Write,
    },
    path::{
        Component,
        Path,
        PathBuf,
    },
};

use crate::PersistenceTransport;

#[derive(Clone, Debug)]
pub struct PersistedLocal {
    path: PathBuf,
}

impl PersistedLocal {
    /// Use an explicit path, including project-specific state files.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Use `<OS config directory>/<app name>/<file name>`.
    pub fn for_app(app_name: &str, file_name: &str) -> io::Result<Self> {
        for name in [app_name, file_name] {
            let mut components = Path::new(name).components();
            if !matches!(components.next(), Some(Component::Normal(_)))
                || components.next().is_some()
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "application and file names must be single path components",
                ));
            }
        }
        let directory = dirs::config_dir().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "OS config directory is unavailable",
            )
        })?;
        Ok(Self::new(directory.join(app_name).join(file_name)))
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl PersistenceTransport for PersistedLocal {
    type Error = io::Error;

    fn load(&self) -> io::Result<Option<Vec<u8>>> {
        match fs::read(&self.path) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error),
        }
    }

    fn save(&self, bytes: &[u8]) -> io::Result<()> {
        let parent = self
            .path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent)?;
        let mut file = tempfile::NamedTempFile::new_in(parent)?;
        file.write_all(bytes)?;
        file.as_file().sync_all()?;
        file.persist(&self.path).map_err(|error| error.error)?;
        Ok(())
    }
}
