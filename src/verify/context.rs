use std::thread;

#[derive(Clone, Debug)]
pub struct TestContext {
    pub module: Vec<String>,
    pub name: String,
}

impl TestContext {
    pub fn new() -> Self {
        let name = thread::current()
            .name()
            .expect("Should be able to get test name")
            .to_owned();
        let components: Vec<String> = name.split("::").map(ToOwned::to_owned).collect();
        let path = components
            .iter()
            .take(components.len() - 1)
            .map(ToOwned::to_owned)
            .collect();
        let name = components
            .last()
            .expect("Should be at least one component in test name")
            .to_owned();
        Self { module: path, name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        // Arrange
        // Act
        let context = TestContext::new();
        // Assert
        assert_eq!(
            context.module,
            vec![
                "verify".to_owned(),
                "context".to_owned(),
                "tests".to_owned()
            ]
        );
        assert_eq!(context.name, "new".to_owned());
    }
}
