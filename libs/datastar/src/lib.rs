use serde::Serialize;
use std::convert::Infallible;

use axum::response::Sse;
use axum::response::sse::Event;
use futures_util::stream;

use crate::namespace::Namespace;
use crate::{elements::DatastarElement, modes::DatastarMode};

use crate::events::DatastarEvent;

pub mod elements;
pub mod events;
pub mod modes;
pub mod namespace;
pub mod query_params;
pub mod templates;

pub fn patch_elements<'a>() -> PatchElementsBuilder {
    PatchElementsBuilder::new()
}

#[derive(Default)]
pub struct PatchElementsBuilder {
    mode: Option<DatastarMode>,
    selector: Option<String>,
    elements: Vec<DatastarElement>,
    namespace: Option<Namespace>,
    use_view_transition: Option<bool>,
}

impl<'a> PatchElementsBuilder {
    pub fn new() -> Self {
        PatchElementsBuilder::default()
    }
    /// Sets the mode
    pub fn mode(mut self, mode: DatastarMode) -> Self {
        self.mode = Some(mode);
        self
    }
    /// Sets the selector
    pub fn selector(mut self, selector: String) -> Self {
        self.selector = Some(selector);
        self
    }
    /// Add an element
    pub fn elements(mut self, element: impl Into<DatastarElement>) -> Self {
        self.elements.push(element.into());
        self
    }
    pub fn use_view_transition(mut self, use_view_transition: bool) -> Self {
        self.use_view_transition = Some(use_view_transition);
        self
    }
    pub fn namespace(mut self, namespace: Namespace) -> Self {
        self.namespace = Some(namespace);
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
            buffer.push_str(&format!("{}\n", element));
        }
        if let Some(use_view_transition) = self.use_view_transition
            && use_view_transition
        {
            buffer.push_str(&format!("useViewTransition {}\n", use_view_transition));
        }
        if let Some(namespace) = self.namespace {
            buffer.push_str(&format!("{}\n", namespace));
        }
        event.data(buffer)
    }
    pub fn axum_stream(self) -> Sse<impl futures_core::Stream<Item = Result<Event, Infallible>>> {
        Sse::new(stream::iter([Ok(self.axum_event())]))
    }
}

impl Into<Event> for PatchElementsBuilder {
    fn into(self) -> Event {
        self.axum_event()
    }
}

pub fn patch_signals<'a, S: Serialize>(signals: S) -> PatchSignalsBuilder<S> {
    PatchSignalsBuilder {
        signals,
        only_if_missing: None,
    }
}

pub struct PatchSignalsBuilder<S: Serialize> {
    signals: S,
    only_if_missing: Option<bool>,
}

impl<'a, S: Serialize> PatchSignalsBuilder<S> {
    /// Sets the signals
    pub fn signals(mut self, signals: S) -> Self {
        self.signals = signals;
        self
    }
    pub fn only_if_missing(mut self, only_if_missing: bool) -> Self {
        self.only_if_missing = Some(only_if_missing);
        self
    }
    pub fn axum_event(self) -> Event {
        let event = Event::default();
        let event = event.event::<&str>(DatastarEvent::DatastarPatchSignals.into());
        let mut buffer = String::new();
        buffer.push_str(&format!(
            "signals {}\n",
            serde_json::to_string(&self.signals).unwrap_or_default()
        ));
        event.data(buffer)
    }
    pub fn axum_stream(self) -> Sse<impl futures_core::Stream<Item = Result<Event, Infallible>>> {
        Sse::new(stream::iter([Ok(self.axum_event())]))
    }
}

impl<S: Serialize> Into<Event> for PatchSignalsBuilder<S> {
    fn into(self) -> Event {
        self.axum_event()
    }
}

pub fn event_stream(
    events: impl IntoIterator<Item = Event>,
) -> Sse<impl futures_core::Stream<Item = Result<Event, Infallible>> + Send> {
    let events: Vec<Event> = events.into_iter().collect();
    let s = stream::iter(events.into_iter().map(Ok));
    Sse::new(s)
}
