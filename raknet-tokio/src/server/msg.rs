use crate::session::RakSession;
use raknet::prelude::RakSession as RakSessionIntl;
use tokio::sync::oneshot;

pub enum RakServerMsg {
    SetMessage(Box<[u8]>),
    SetMaxConnections(usize),
    Stop,
    /// See `RakServer::adopt_session` (core crate) - a session exported from another
    /// `RakServer` instance, to be adopted here and continue seamlessly. Replied to
    /// directly via the oneshot (not the normal `accept()` stream) so the caller gets
    /// the resumed session handle without racing a real handshake landing at the same
    /// moment.
    AdoptSession(RakSessionIntl, oneshot::Sender<RakSession>),
}
