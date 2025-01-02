use futures_util::{SinkExt, StreamExt};
use log::*;
use std::net::SocketAddr;
use std::time::SystemTime;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Result},
};
use tracing::info;

use {anyhow, discord_sdk as ds, tokio, tracing};

const APP_ID: ds::AppId = 1300015589540106240;

pub struct Client {
    pub discord: ds::Discord,
    pub user: ds::user::User,
    pub wheel: ds::wheel::Wheel,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // connect to client
    let client = make_client(ds::Subscriptions::ACTIVITY).await;
    let mut activity_events = client.wheel.activity();

    tokio::task::spawn(async move {
        while let Ok(ae) = activity_events.0.recv().await {
            tracing::info!(event = ?ae, "received activity event");
        }
    });

    // build activity
    let rp = ds::activity::ActivityBuilder::default()
        .details("song".to_owned())
        .state("on platform".to_owned())
        .start_timestamp(SystemTime::now());

    // update activity
    tracing::info!(
        "updated activity: {:?}",
        client.discord.update_activity(rp).await
    );

    // open websocket server
    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await.expect("can't listen");
    info!("websocket listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("peer address is {}", peer);

        tokio::spawn(accept_connection(peer, stream));
    }

    // read_line to prevent main from terminating
    let mut r = String::new();
    let _ = std::io::stdin().read_line(&mut r);

    // after termination signal
    tracing::info!(
        "cleared activity: {:?}",
        client.discord.clear_activity().await
    );

    client.discord.disconnect().await;

    Ok(())
}

pub async fn make_client(subs: ds::Subscriptions) -> Client {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let (wheel, handler) = ds::wheel::Wheel::new(Box::new(|err| {
        tracing::error!(error = ?err, "encountered an error");
    }));

    let mut user = wheel.user();

    let discord = ds::Discord::new(ds::DiscordApp::PlainId(APP_ID), subs, Box::new(handler))
        .expect("unable to create discord client");

    tracing::info!("waiting for handshake...");
    user.0.changed().await.unwrap();

    let user = match &*user.0.borrow() {
        ds::wheel::UserState::Connected(user) => user.clone(),
        ds::wheel::UserState::Disconnected(err) => panic!("failed to connect to Discord: {}", err),
    };

    tracing::info!("connected to discord, local user is {:#?}", user);

    Client {
        discord,
        user,
        wheel,
    }
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    info!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            ws_stream.send(msg).await?;
        }
    }

    Ok(())
}
