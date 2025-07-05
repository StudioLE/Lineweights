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

    pub fn string(&mut self, received: &String, extension: &str) -> Result<bool, VerifyError> {
        self.verify_dirs()?;
        self.context.write_received_text(received, extension)?;
        let verified = self.context.read_verified_text(extension)?;
        Ok(diff_string(received, &verified))
    }

    pub fn value<T: Debug + DeserializeOwned + PartialEq + Serialize>(
        &mut self,
        received: T,
    ) -> Result<bool, VerifyError> {
        self.verify_dirs()?;
        self.context.write_received_json(&received)?;
        let verified = self.context.read_verified_json::<T>()?;
        Ok(diff_value(received, verified))
    }

    pub fn values<T: Debug + DeserializeOwned + PartialEq + Serialize>(
        &mut self,
        received: &[T],
    ) -> Result<bool, VerifyError> {
        self.verify_dirs()?;
        self.context.write_received_json(&received)?;
        let verified = self.context.read_verified_json::<Vec<T>>()?;
        Ok(diff_values(received, verified))
    }

    fn verify_dirs(&mut self) -> Result<(), VerifyError> {
        let dir = TestContext::get_verify_dir();
        if !dir.is_dir() {
            return Err(VerifyError::VerifyDirectoryNotFound);
        }
        let dir = self.context.get_path();
        let dir = dir.parent().expect("Parent directory should exist");
        if !dir.is_dir() {
            create_dir_all(dir).map_err(VerifyError::File)?;
        }
        Ok(())
    }
}

fn diff_values<T: Debug + DeserializeOwned + PartialEq + Serialize>(
    received: &[T],
    verified: Vec<T>,
) -> bool {
    let max = if received.len() > verified.len() {
        received.len()
    } else {
        verified.len()
    };
    let mut is_success = true;
    for i in 0..max {
        let Some(received_item) = received.get(i) else {
            println!("Missing received item at index {i}");
            let verified_item = verified.get(i).expect("Verified item should exist");
            println!("Expected: {}", display_value(&verified_item));
            is_success = false;
            continue;
        };
        let Some(verified_item) = verified.get(i) else {
            println!("Missing verified item at index {i}");
            println!("Received: {}", display_value(received_item));
            is_success = false;
            continue;
        };
        if received_item != verified_item {
            println!("Failed at index {i}");
            println!("Expected: {}", display_value(&verified_item));
            println!("Received: {}", display_value(&received_item));
            is_success = false;
        }
    }
    is_success
}

fn diff_string(received: &String, verified: &String) -> bool {
    let is_success = received == verified;
    if !is_success {
        println!("Expected: {verified}");
        println!("Received: {received}");
    }
    is_success
}

fn diff_value<T: Debug + DeserializeOwned + PartialEq + Serialize>(
    received: T,
    verified: T,
) -> bool {
    let is_success = received == verified;
    if !is_success {
        println!("Expected: {}", display_value(&verified));
        println!("Received: {}", display_value(&received));
    }
    is_success
}

fn display_value<T: Debug + Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| format!("{value:?}"))
}
