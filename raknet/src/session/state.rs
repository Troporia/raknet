#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash, serde::Serialize, serde::Deserialize)]
pub enum RakSessionState {
    Connected,
    Disconnected,
}
