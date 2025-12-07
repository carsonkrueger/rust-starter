use crate::services::New;

pub trait HelloWorldService {
    async fn hello_world(&self) -> String;
}

#[derive(Clone, Debug)]
pub struct HelloWorld {}

impl New for HelloWorld {
    fn new() -> Self {
        Self {}
    }
}

impl HelloWorldService for HelloWorld {
    async fn hello_world(&self) -> String {
        "Hello, World!".to_string()
    }
}
