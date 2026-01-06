use core::fmt;
use std::{fmt::Display, ops::Add};

use axum::response::sse::Event;
use strum::IntoStaticStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoStaticStr)]
pub enum DatastarEvent {
    DatastarPatchElements,
    DatastarPatchSignals,
}

pub struct DatastarElement(pub String);

impl AsRef<str> for DatastarElement {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Into<Event> for DatastarElement {
    fn into(self) -> Event {
        Event::default()
            .event::<&'static str>(DatastarEvent::DatastarPatchElements.into())
            .data::<String>(self.into())
    }
}

impl Display for DatastarElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let split = self.0.split("\n");
        for line in split {
            f.write_str("elements ")?;
            f.write_str(line)?;
        }
        Ok(())
    }
}

impl DatastarElement {
    pub fn redirect(to: &str) -> Result<Self, fmt::Error> {
        let mut writer = String::new();
        templr::write_escaped(&mut writer, &to)?;
        Ok(Self(format!(
            "<script>window.location.href = \"{}\"</script>",
            writer
        )))
    }
    pub fn event_with_mode(&self, selector: &str, mode: DatastarMode) -> Event {
        Event::default()
            .event::<&'static str>(DatastarEvent::DatastarPatchElements.into())
            .data::<String>(format!("selector {}\n{}\n{}", selector, mode, self))
    }
    pub fn event(self) -> Event {
        self.into()
    }
}

impl Into<String> for DatastarElement {
    fn into(self) -> String {
        format!("{}", self.0)
    }
}

impl Add<DatastarElement> for DatastarElement {
    type Output = Self;

    fn add(self, rhs: DatastarElement) -> Self::Output {
        DatastarElement(self.0 + &rhs.0)
    }
}

impl Add<&str> for DatastarElement {
    type Output = Self;

    fn add(self, rhs: &str) -> Self::Output {
        DatastarElement(self.0 + rhs)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoStaticStr)]
pub enum DatastarMode {
    Outer,
    Inner,
    Replace,
    Prepend,
    Append,
    Before,
    After,
    Remove,
}

impl Display for DatastarMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("mode ")?;
        f.write_str(self.into())
    }
}
