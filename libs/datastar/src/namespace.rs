use std::fmt::Display;

pub enum Namespace {
    Svg,
    Mathml,
}

impl Into<&'static str> for &Namespace {
    fn into(self) -> &'static str {
        match self {
            Namespace::Svg => "svg",
            Namespace::Mathml => "mathml",
        }
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: &'static str = self.into();
        f.write_str(str)
    }
}
