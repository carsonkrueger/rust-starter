use std::fmt::Display;

use strum::IntoStaticStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoStaticStr)]
pub enum DatastarEvent {
    DatastarPatchElements,
    DatastarPatchSignals,
}

pub struct DatastarElements<'a>(pub &'a str);

impl Display for DatastarElements<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("elements ")?;
        f.write_str(self.0)?;
        f.write_str("\n")
    }
}

impl Into<String> for DatastarElements<'_> {
    fn into(self) -> String {
        format!("{}", self.0)
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
