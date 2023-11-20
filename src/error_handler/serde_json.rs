pub fn from_str<'a, T>(s: &'a str) -> T
where
    T: serde::de::Deserialize<'a>,
{
    match serde_json::from_str::<'a, T>(s) {
        Ok(v) => v,
        Err(err) => match err.io_error_kind() {
            Some(err_kind) => {
                match err_kind {
                    std::io::ErrorKind::NotFound => todo!(),
                    std::io::ErrorKind::PermissionDenied => todo!(),
                    std::io::ErrorKind::ConnectionRefused => todo!(),
                    std::io::ErrorKind::ConnectionReset => todo!(),
                    std::io::ErrorKind::HostUnreachable => todo!(),
                    std::io::ErrorKind::NetworkUnreachable => todo!(),
                    std::io::ErrorKind::ConnectionAborted => todo!(),
                    std::io::ErrorKind::NotConnected => todo!(),
                    std::io::ErrorKind::AddrInUse => todo!(),
                    std::io::ErrorKind::AddrNotAvailable => todo!(),
                    std::io::ErrorKind::NetworkDown => todo!(),
                    std::io::ErrorKind::BrokenPipe => todo!(),
                    std::io::ErrorKind::AlreadyExists => todo!(),
                    std::io::ErrorKind::WouldBlock => todo!(),
                    std::io::ErrorKind::NotADirectory => todo!(),
                    std::io::ErrorKind::IsADirectory => todo!(),
                    std::io::ErrorKind::DirectoryNotEmpty => todo!(),
                    std::io::ErrorKind::ReadOnlyFilesystem => todo!(),
                    std::io::ErrorKind::FilesystemLoop => todo!(),
                    std::io::ErrorKind::StaleNetworkFileHandle => todo!(),
                    std::io::ErrorKind::InvalidInput => todo!(),
                    std::io::ErrorKind::InvalidData => todo!(),
                    std::io::ErrorKind::TimedOut => todo!(),
                    std::io::ErrorKind::WriteZero => todo!(),
                    std::io::ErrorKind::StorageFull => todo!(),
                    std::io::ErrorKind::NotSeekable => todo!(),
                    std::io::ErrorKind::FilesystemQuotaExceeded => todo!(),
                    std::io::ErrorKind::FileTooLarge => todo!(),
                    std::io::ErrorKind::ResourceBusy => todo!(),
                    std::io::ErrorKind::ExecutableFileBusy => todo!(),
                    std::io::ErrorKind::Deadlock => todo!(),
                    std::io::ErrorKind::CrossesDevices => todo!(),
                    std::io::ErrorKind::TooManyLinks => todo!(),
                    std::io::ErrorKind::InvalidFilename => todo!(),
                    std::io::ErrorKind::ArgumentListTooLong => todo!(),
                    std::io::ErrorKind::Interrupted => todo!(),
                    std::io::ErrorKind::Unsupported => todo!(),
                    std::io::ErrorKind::UnexpectedEof => todo!(),
                    std::io::ErrorKind::OutOfMemory => todo!(),
                    std::io::ErrorKind::Other => todo!(),
                    _ => todo!(),
                }
            }
            None => {
                eprintln!("Error: {:?}", err);
                eprintln!("Value input: {:?}", s);
                unimplemented!();
            }
        }
    }
}
