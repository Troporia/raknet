use raknet::prelude::{RakPriority, RakReliability, RakSession as RakSessionIntl, RakSessionError};
use tokio::sync::oneshot::Sender;

pub enum RakSessionMsg {
    Send(
        Box<[u8]>,
        RakReliability,
        RakPriority,
        Sender<Result<(), RakSessionError>>,
    ),
    Close(Sender<Result<(), RakSessionError>>),
    IsClosed(Sender<bool>),
    /// Snapshot the session's live protocol state for a cross-process handoff - see
    /// `RakSession::export_state`. Cloning is cheap relative to a network round trip
    /// and leaves the live task's own session untouched/still running, so an export
    /// that's never actually followed by a handoff (e.g. the target never came up)
    /// costs nothing beyond the clone itself.
    ExportState(Sender<RakSessionIntl>),
}
