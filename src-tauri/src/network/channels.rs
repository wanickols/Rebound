use std::net::SocketAddr;

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::network::{
    clientid::ClientId,
    clientrequest::{ClientMessage, ClientRequest},
    serverevent::ServerEvent,
    socketmanager::SocketData,
};

// Struct to hold all channels
pub struct HostChannelReceivers {
    pub snapshot_rx: UnboundedReceiver<ServerEvent>,
    pub game_rx: UnboundedReceiver<(ClientRequest, ClientId)>,
    pub client_message_rx: UnboundedReceiver<ClientMessage>,
    pub client_event_rx: UnboundedReceiver<ServerEvent>,
    pub frontend_request_rx: UnboundedReceiver<ClientRequest>,

    pub incoming_socket_data_rx: UnboundedReceiver<SocketData>,
    pub outgoing_socket_data_rx: UnboundedReceiver<SocketData>,

    pub client_tick_rx: UnboundedReceiver<ClientId>,
    pub client_dead_rx: UnboundedReceiver<ClientId>,

    pub shutdown_rx: tokio::sync::watch::Receiver<bool>,
}

pub struct HostChannelSenders {
    pub snapshot_tx: UnboundedSender<ServerEvent>,
    pub game_tx: UnboundedSender<(ClientRequest, ClientId)>,
    pub client_message_tx: UnboundedSender<ClientMessage>,
    pub client_event_tx: UnboundedSender<ServerEvent>,
    pub frontend_request_tx: UnboundedSender<ClientRequest>,

    pub incoming_socket_data_tx: UnboundedSender<SocketData>,
    pub outgoing_socket_data_tx: UnboundedSender<SocketData>,

    pub client_tick_tx: UnboundedSender<ClientId>,
    pub client_dead_tx: UnboundedSender<ClientId>,

    pub shutdown_tx: tokio::sync::watch::Sender<bool>,
}

pub fn init_channels() -> (HostChannelSenders, HostChannelReceivers) {
    //Gm
    let (snapshot_tx, snapshot_rx) = unbounded_channel::<ServerEvent>();
    let (game_tx, game_rx) = unbounded_channel::<(ClientRequest, ClientId)>();

    //Network
    let (client_message_tx, client_message_rx) = unbounded_channel::<ClientMessage>();
    let (client_event_tx, client_event_rx) = unbounded_channel::<ServerEvent>();
    let (frontend_request_tx, frontend_request_rx) = unbounded_channel::<ClientRequest>();

    //TTL
    let (client_tick_tx, client_tick_rx) = unbounded_channel::<ClientId>();
    let (client_dead_tx, client_dead_rx) = unbounded_channel::<ClientId>();

    //Socket
    let (incoming_socket_data_tx, incoming_socket_data_rx) = unbounded_channel::<SocketData>();
    let (outgoing_socket_data_tx, outgoing_socket_data_rx) = unbounded_channel::<SocketData>();

    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);

    let senders = HostChannelSenders {
        snapshot_tx,
        game_tx,
        client_message_tx,
        client_event_tx,
        frontend_request_tx,
        incoming_socket_data_tx,
        outgoing_socket_data_tx,
        client_tick_tx,
        client_dead_tx,
        shutdown_tx,
    };

    let receivers = HostChannelReceivers {
        snapshot_rx,
        game_rx,
        client_message_rx,
        client_event_rx,
        frontend_request_rx,
        incoming_socket_data_rx,
        outgoing_socket_data_rx,
        client_tick_rx,
        client_dead_rx,
        shutdown_rx,
    };

    (senders, receivers)
}
