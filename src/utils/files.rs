use std::{fs::File, path::PathBuf};

pub trait FileishExt {
    fn readable(&self) -> File;

    fn try_readable(&self) -> Result<File, std::io::Error>;

    fn appendable(&self) -> File;

    fn try_appendable(&self) -> Result<File, std::io::Error>;

    fn truncatable(&self) -> File;

    fn try_truncatable(&self) -> Result<File, std::io::Error>;

    fn create_all_dirs(&self) -> Result<(), std::io::Error>;
}

impl FileishExt for PathBuf {
    fn readable(&self) -> File {
        File::options().read(true).open(self).unwrap()
    }

    fn try_readable(&self) -> Result<File, std::io::Error> {
        File::options().read(true).open(self)
    }

    fn truncatable(&self) -> File {
        File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self)
            .unwrap()
    }

    fn try_truncatable(&self) -> Result<File, std::io::Error> {
        File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self)
    }

    fn create_all_dirs(&self) -> Result<(), std::io::Error> {
        if !self.exists() {
            std::fs::create_dir_all(self)
        } else {
            Ok(())
        }
    }

    fn appendable(&self) -> File {
        File::options()
            .create(true)
            .write(true)
            .append(true)
            .open(self)
            .unwrap()
    }

    fn try_appendable(&self) -> Result<File, std::io::Error> {
        File::options()
            .create(true)
            .write(true)
            .append(true)
            .open(self)
    }
}
