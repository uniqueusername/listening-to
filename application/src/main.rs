use std::time::SystemTime;
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
    let rp = ds::activity::ActivityBuilder::default().start_timestamp(SystemTime::now());

    // update activity
    tracing::info!(
        "updated activity: {:?}",
        client.discord.update_activity(rp).await
    );

    let mut r = String::new();
    let _ = std::io::stdin().read_line(&mut r);

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
