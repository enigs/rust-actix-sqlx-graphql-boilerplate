use actix_web::rt::time::interval;
use actix_web_lab::sse::{self, Sse};
use actix_web_lab::util::InfallibleStream;
use futures_util::future;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub use actix_web_lab::extract::Path;

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct BroadcasterInner {
    clients: HashMap<String, Vec<mpsc::Sender<sse::Event>>>,
}

impl Broadcaster {
    /// Constructs new broadcaster and spawns ping loop.
    pub fn create() -> Arc<Self> {
        let this = Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        });

        Broadcaster::spawn_ping(Arc::clone(&this));

        this
    }

    /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast
    /// list if not.
    fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    /// Removes all non-responsive clients from broadcast list.
    async fn remove_stale_clients(&self) {
        let client_map = self.inner.lock().clients.clone();

        let mut ok_client_map = HashMap::new();

        for (id, clients) in client_map {
            let mut ok_clients = Vec::new();

            for client in clients {
                if client
                    .send(sse::Event::Comment("ping".into()))
                    .await
                    .is_ok()
                {
                    ok_clients.push(client);
                }
            }

            if !ok_clients.is_empty() {
                ok_client_map.insert(id, ok_clients);
            }
        }

        self.inner.lock().clients = ok_client_map;
    }

    /// Registers client with broadcaster, returning an SSE response body.
    pub async fn new_client(&self, id: &str) -> Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
        let (tx, rx) = mpsc::channel(10);

        tx.send(sse::Data::new("connected").into()).await.unwrap();

        self.inner.lock().clients
            .entry(id.into())
            .or_default()
            .push(tx);

        Sse::from_infallible_receiver(rx)
    }

    /// Broadcasts `msg` to all clients.
    pub async fn broadcast(&self, channel: &str, ty: &str, message: &str) {
        let client_map = self.inner.lock().clients.clone();

        if let Some(clients) = client_map.get(channel) {
            let send_futures = clients
                .iter()
                .map(|client| client.send(sse::Data::new(message).event(ty).into()));

            // try to send to all clients, ignoring failures
            // disconnected clients will get swept up by `remove_stale_clients`
            let _ = future::join_all(send_futures).await;
        }
    }
}