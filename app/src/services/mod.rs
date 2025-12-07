use crate::services::hello_world::HelloWorldService;

pub mod hello_world;

pub trait New {
    fn new() -> Self;
}

#[derive(Clone, Debug)]
pub struct ServiceManager<HWS: HelloWorldService> {
    pub hello_world: HWS,
}

impl<HW> Default for ServiceManager<HW>
where
    HW: New + HelloWorldService,
{
    fn default() -> Self {
        Self {
            hello_world: HW::new(),
        }
    }
}
