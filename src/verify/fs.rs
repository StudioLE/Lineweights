use crate::verify::prelude::*;
use serde::Serialize;
pub use std::io::{BufReader, BufWriter};
use std::io::{Read, Write};

pub(crate) const VERIFY_DIR: &str = "verify";
pub(crate) const JSON_EXT: &str = "json";
pub(crate) const RECEIVED_EXT: &str = "received";
pub(crate) const VERIFIED_EXT: &str = "verified";

impl TestContext {
    pub(super) fn get_verify_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(VERIFY_DIR)
    }

    pub(super) fn get_path(&self) -> PathBuf {
        let mut path = Self::get_verify_dir();
        for component in &self.module {
            path.push(component);
        }
        path.push(self.name.clone());
        path
    }

    fn get_received_path(&self, extension: &str) -> PathBuf {
        let mut path = self.get_path();
        path.set_extension(format!("{RECEIVED_EXT}.{extension}"));
        path
    }

    fn get_verified_path(&self, extension: &str) -> PathBuf {
        let mut path = self.get_path();
        path.set_extension(format!("{VERIFIED_EXT}.{extension}"));
        path
    }

    pub(super) fn read_verified_text(&mut self, extension: &str) -> Result<String, VerifyError> {
        let mut reader = self.get_verified_reader(extension)?;
        let mut text = String::new();
        reader
            .read_to_string(&mut text)
            .map_err(VerifyError::Buffer)?;
        Ok(text)
    }

    pub(super) fn read_verified_json<T: DeserializeOwned>(&mut self) -> Result<T, VerifyError> {
        let reader = self.get_verified_reader(JSON_EXT)?;
        serde_json::from_reader(reader).map_err(VerifyError::Deserialization)
    }

    pub(super) fn write_received_text(
        &mut self,
        received: &String,
        extension: &str,
    ) -> Result<(), VerifyError> {
        let mut writer = self.get_received_writer(extension)?;
        writer
            .write_all(received.as_bytes())
            .map_err(VerifyError::Buffer)?;
        Ok(())
    }

    pub(super) fn write_received_json<T: Serialize>(
        &mut self,
        received: &T,
    ) -> Result<(), VerifyError> {
        let mut writer = self.get_received_writer(JSON_EXT)?;
        serde_json::to_writer_pretty(&mut writer, received).map_err(VerifyError::Serialization)?;
        writer.flush().map_err(VerifyError::Buffer)?;
        Ok(())
    }

    fn get_verified_reader(&mut self, extension: &str) -> Result<BufReader<File>, VerifyError> {
        let path = self.get_verified_path(extension);
        if !path.is_file() {
            println!("Creating verified file: {}", path.display());
            let received = self.get_received_path(extension);
            copy(received, &path).map_err(VerifyError::File)?;
        }
        let file = File::open(path).map_err(VerifyError::File)?;
        Ok(BufReader::new(file))
    }

    fn get_received_writer(&mut self, extension: &str) -> Result<BufWriter<File>, VerifyError> {
        let path = self.get_received_path(extension);
        let file = File::create(path).map_err(VerifyError::File)?;
        Ok(BufWriter::new(file))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_path() {
        // Arrange
        let context = TestContext::new();
        // Act
        let mut path = context.get_path();
        // Assert
        assert_eq!(
            path,
            TestContext::get_verify_dir().join("verify/fs/tests/get_path")
        );
        // Arrange
        path.set_extension(format!("{VERIFIED_EXT}.{JSON_EXT}"));
        assert_eq!(
            path,
            TestContext::get_verify_dir().join(format!(
                "verify/fs/tests/get_path.{VERIFIED_EXT}.{JSON_EXT}"
            ))
        );
    }
}
