pub mod client;
pub mod server;

pub mod prelude {
    pub use crate::client::{RakClient, RakClientEvent, RakClientPlugin, RakClientSet};
    pub use crate::server::{RakServer, RakServerEvent, RakServerPlugin, RakServerSet};
    pub use raknet::prelude::{
        RakClientConfig, RakDisconnectReason, RakPriority, RakReliability, RakServerConfig,
        RakSessionError, RakSessionId,
    };
}
