use strum::IntoStaticStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum DatastarEvent {
    DatastarPatchElements,
    DatastarPatchSignals,
}
