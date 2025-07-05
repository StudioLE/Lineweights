use crate::verify::prelude::*;
use serde::Serialize;
use std::io::Write;
pub use std::io::{BufReader, BufWriter};

pub(crate) const VERIFY_DIR: &str = "verify";
pub(crate) const JSON_EXT: &str = "json";
pub(crate) const RECEIVED_EXT: &str = "received";
pub(crate) const VERIFIED_EXT: &str = "verified";

impl TestContext {
    pub(super) fn get_verify_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(VERIFY_DIR)
    }

    pub(super) fn get_received_path(&self) -> PathBuf {
        let mut path = self.get_path();
        path.set_extension(format!("{RECEIVED_EXT}.{JSON_EXT}"));
        path
    }

    pub(super) fn get_verified_path(&self) -> PathBuf {
        let mut path = self.get_path();
        path.set_extension(format!("{VERIFIED_EXT}.{JSON_EXT}"));
        path
    }

    pub(super) fn read_verified<T: DeserializeOwned>(&mut self) -> Result<Vec<T>, VerifyError> {
        let path = self.get_verified_path();
        if !path.is_file() {
            return Err(VerifyError::VerifiedFileNotFound);
        }
        let file = File::open(path).map_err(VerifyError::File)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).map_err(VerifyError::Deserialization)
    }

    pub(super) fn write_received<T: Serialize>(&mut self, received: T) -> Result<(), VerifyError> {
        let path = self.get_received_path();
        let file = File::create(path).map_err(VerifyError::File)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &received).map_err(VerifyError::Serialization)?;
        writer.flush().map_err(VerifyError::Buffer)?;
        Ok(())
    }
}
