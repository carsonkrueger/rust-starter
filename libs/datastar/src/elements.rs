use core::fmt;
use std::{fmt::Display, ops::Add};

pub struct DatastarElement(pub String);

impl AsRef<str> for DatastarElement {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for DatastarElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.0.split("\n") {
            f.write_str("elements ")?;
            f.write_str(line)?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl DatastarElement {
    pub fn redirect_element(to: &str) -> Result<Self, fmt::Error> {
        Ok(Self(format!(
            "<script>window.location.href = \"{}\"</script>",
            to
        )))
    }
}

impl Into<String> for DatastarElement {
    fn into(self) -> String {
        format!("{}", self.0)
    }
}

impl From<&str> for DatastarElement {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for DatastarElement {
    fn from(value: String) -> Self {
        Self(value)
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

impl Add<String> for DatastarElement {
    type Output = Self;

    fn add(self, rhs: String) -> Self::Output {
        DatastarElement(self.0 + &rhs)
    }
}
