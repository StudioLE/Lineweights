use std::io;

#[derive(Debug)]
pub enum VerifyError {
    VerifyDirectoryNotFound,
    VerifiedFileNotFound,
    RecievedFileNotFound,
    #[allow(clippy::absolute_paths)]
    File(io::Error),
    #[allow(clippy::absolute_paths)]
    Deserialization(serde_json::Error),
    #[allow(clippy::absolute_paths)]
    Serialization(serde_json::Error),
    #[allow(clippy::absolute_paths)]
    Buffer(io::Error),
}
