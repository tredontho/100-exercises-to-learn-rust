use std::sync::Arc;

use outro_08::{
    data::{TicketDescription, TicketDraft, TicketTitle},
    store::TicketStore,
};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let mut store = TicketStore::new();
    store.add_ticket(TicketDraft {
        title: TicketTitle::try_from("Test Title").unwrap(),
        description: TicketDescription::try_from("Test Description").unwrap(),
    });
    // let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    let api = filters::tickets(Arc::new(RwLock::new(store)));

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await
}

mod filters {
    use std::{convert::Infallible, sync::Arc};

    use super::handlers;
    use outro_08::{
        data::TicketDraft,
        store::{TicketId, TicketStore},
    };
    use tokio::sync::RwLock;
    use warp::Filter;
    pub fn tickets(
        store: Arc<RwLock<TicketStore>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        ticket_create(store.clone()).or(ticket_get(store.clone()))
    }

    pub fn ticket_create(
        store: Arc<RwLock<TicketStore>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("tickets")
            .and(warp::post())
            .and(warp::body::json())
            .and(with_store(store))
            .and_then(handlers::create_ticket)
    }
    pub fn ticket_get(
        store: Arc<RwLock<TicketStore>>,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("tickets" / u64)
            .map(|id| TicketId(id))
            .and(warp::get())
            .and(with_store(store))
            .and_then(handlers::get_ticket)
    }

    fn with_store(
        store: Arc<RwLock<TicketStore>>,
    ) -> impl Filter<Extract = (Arc<RwLock<TicketStore>>,), Error = Infallible> + Clone {
        warp::any().map(move || store.clone())
    }
}

mod handlers {
    use std::borrow::Borrow;
    use std::convert::Infallible;
    use std::ops::Deref;
    use std::sync::Arc;

    use outro_08::data::TicketDraft;
    use outro_08::store::{TicketId, TicketStore};
    use tokio::sync::RwLock;
    // create, retrieve, patch
    pub async fn create_ticket(
        draft: TicketDraft,
        store: Arc<RwLock<TicketStore>>,
    ) -> Result<warp::reply::Json, Infallible> {
        let mut store = store.write().await;
        let id = store.add_ticket(draft);
        Ok(warp::reply::json(&id.0))
    }

    pub async fn get_ticket(
        id: TicketId,
        store: Arc<RwLock<TicketStore>>,
    ) -> Result<warp::reply::Json, Infallible> {
        let store = store.read().await;
        let ticket = store.get(id).unwrap();
        let read = &ticket.read().await;
        let ticket = read.deref();
        Ok(warp::reply::json(ticket))
    }
}
