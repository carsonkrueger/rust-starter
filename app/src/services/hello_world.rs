use tracing::trace;

pub trait HelloWorldService {
    fn new() -> Self;
    async fn hello_world(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct HelloWorld {}

impl HelloWorldService for HelloWorld {
    fn new() -> Self {
        Self {}
    }
    async fn hello_world(&self) -> String {
        trace!("->> hello_world");
        "Hello World!".into()
    }
}
