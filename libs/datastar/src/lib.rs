use std::convert::Infallible;

use axum::response::Sse;
use axum::response::sse::Event;
use futures_util::stream;

use crate::{elements::DatastarElement, modes::DatastarMode};

use crate::events::DatastarEvent;

pub mod elements;
pub mod events;
pub mod modes;

pub fn patch_elements<'a>() -> PatchElementsBuilder<'a> {
    PatchElementsBuilder::new()
}

#[derive(Default)]
pub struct PatchElementsBuilder<'a> {
    mode: Option<DatastarMode>,
    selector: Option<&'a str>,
    elements: Vec<DatastarElement>,
}

impl<'a> PatchElementsBuilder<'a> {
    pub fn new() -> Self {
        PatchElementsBuilder::default()
    }
    /// Sets the mode
    pub fn mode(mut self, mode: DatastarMode) -> Self {
        self.mode = Some(mode);
        self
    }
    /// Sets the selector
    pub fn selector(mut self, selector: &'a str) -> Self {
        self.selector = Some(selector);
        self
    }
    /// Add an element
    pub fn elements(mut self, element: impl Into<DatastarElement>) -> Self {
        self.elements.push(element.into());
        self
    }
    pub fn axum_event(self) -> Event {
        let event = Event::default().event::<&str>(DatastarEvent::DatastarPatchElements.into());
        let mut buffer = String::new();
        if let Some(selector) = self.selector {
            buffer.push_str(&format!("selector {}\n", selector));
        }
        if let Some(mode) = self.mode {
            buffer.push_str(&format!("{}\n", mode));
        }
        for element in self.elements {
            buffer.push_str(&format!("{}", element));
        }
        event.data(buffer)
    }
    pub fn axum_stream(self) -> Sse<impl futures_core::Stream<Item = Result<Event, Infallible>>> {
        Sse::new(stream::iter([Ok(self.axum_event())]))
    }
}
