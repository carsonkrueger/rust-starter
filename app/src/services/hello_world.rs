use async_trait::async_trait;

#[async_trait]
pub trait HelloWorldService {
    fn new() -> Self;
    async fn hello_world(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct HelloWorld {}

#[async_trait]
impl HelloWorldService for HelloWorld {
    fn new() -> Self {
        Self {}
    }
    async fn hello_world(&self) -> String {
        "Hello World!".into()
    }
}
