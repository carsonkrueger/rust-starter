use std::fmt::Display;

use strum::IntoStaticStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
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
