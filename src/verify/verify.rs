use crate::verify::prelude::*;
use std::fmt::Debug;

pub struct Verify {
    context: TestContext,
    count: u32,
}

impl Verify {
    pub fn new() -> Self {
        Self {
            context: TestContext::new(),
            count: 0,
        }
    }

    pub fn multiple<T: Debug + DeserializeOwned + PartialEq + Serialize>(
        &mut self,
        received: &[T],
    ) -> Result<bool, VerifyError> {
        let dir = TestContext::get_verify_dir();
        if !dir.is_dir() {
            return Err(VerifyError::VerifyDirectoryNotFound);
        }
        let dir = self.context.get_path();
        let dir = dir.parent().expect("Parent directory should exist");
        if !dir.is_dir() {
            create_dir_all(dir).map_err(VerifyError::File)?;
        }
        self.context.write_received(received)?;
        let verified = match self.context.read_verified::<T>() {
            Ok(verified) => verified,
            Err(VerifyError::VerifiedFileNotFound) => {
                println!("Verified file does not exist");
                return Ok(false);
            }
            Err(e) => return Err(e),
        };
        let max = if received.len() > verified.len() {
            received.len()
        } else {
            verified.len()
        };
        let mut is_success = true;
        for i in 0..max {
            let Some(received_item) = received.get(i) else {
                println!("Missing received item at index {i}");
                println!(
                    "Expected: {:?}",
                    verified.get(i).expect("Verified item should exist")
                );
                is_success = false;
                continue;
            };
            let Some(verified_item) = verified.get(i) else {
                println!("Missing verified item at index {i}");
                println!("Expected: {received_item:?}");
                is_success = false;
                continue;
            };
            if received_item != verified_item {
                println!("Failed at index {i}");
                println!("Expected: {verified_item:?}");
                println!("Received: {received_item:?}");
                is_success = false;
            }
        }
        Ok(is_success)
    }
}
