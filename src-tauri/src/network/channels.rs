use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::network::{
    clientid::ClientId,
    clientrequest::{ClientMessage, ClientRequest},
    serverevent::ServerEvent,
};

// Struct to hold all channels
pub struct HostChannelReceivers {
    pub snapshot_rx: UnboundedReceiver<ServerEvent>,
    pub game_rx: UnboundedReceiver<(ClientRequest, ClientId)>,
    pub client_message_rx: UnboundedReceiver<ClientMessage>,
    pub client_event_rx: UnboundedReceiver<ServerEvent>,
    pub frontend_request_rx: UnboundedReceiver<ClientRequest>,
}

pub struct HostChannelSenders {
    pub snapshot_tx: UnboundedSender<ServerEvent>,
    pub game_tx: UnboundedSender<(ClientRequest, ClientId)>,
    pub client_message_tx: UnboundedSender<ClientMessage>,
    pub client_event_tx: UnboundedSender<ServerEvent>,
    pub frontend_request_tx: UnboundedSender<ClientRequest>,
}

pub fn init_channels() -> (HostChannelSenders, HostChannelReceivers) {
    let (snapshot_tx, snapshot_rx) = unbounded_channel::<ServerEvent>();
    let (game_tx, game_rx) = unbounded_channel::<(ClientRequest, ClientId)>();
    let (client_message_tx, client_message_rx) = unbounded_channel::<ClientMessage>();
    let (client_event_tx, client_event_rx) = unbounded_channel::<ServerEvent>();
    let (frontend_request_tx, frontend_request_rx) = unbounded_channel::<ClientRequest>();

    let senders = HostChannelSenders {
        snapshot_tx,
        game_tx,
        client_message_tx,
        client_event_tx,
        frontend_request_tx,
    };

    let receivers = HostChannelReceivers {
        snapshot_rx,
        game_rx,
        client_message_rx,
        client_event_rx,
        frontend_request_rx,
    };

    (senders, receivers)
}
