#![feature(let_chains)]

use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod li;
mod ping;
// #![feature(pointer_is_aligned_to)]

// use ferroc::Ferroc;
//
// #[global_allocator]
// static FERROC: Ferroc = Ferroc;
use std::{fs::canonicalize, path::Path, sync::Arc};

use alive_plugin::alter::Alter;
use aok::{Result, OK};
use axum::{
  extract::State,
  middleware,
  routing::{get, post},
  Router,
};
use clap::arg;
use cmdv::cmdv;
pub use ping::ping;
use tower::ServiceBuilder;

pub type Api = State<Arc<alive::Api>>;

// #[derive(Clone)]
// pub struct Share {
//   n: u64,
// }

async fn run(conf_dir: impl AsRef<Path>, port: u16) -> Result<()> {
  let conf_dir = canonicalize(conf_dir.as_ref())?;
  let mut alive = alive::Alive::<Alter>::load(&conf_dir).await?;
  let api = &alive.api;
  let app = Router::new()
    .route("/ping", get(re::FnAny(ping)).with_state(api.clone()))
    .route("/Li", post(re::FnAny(li::post)).with_state(api.clone()))
    .layer(ServiceBuilder::new().layer(middleware::from_fn(axum_cors::cors)));
  tokio::spawn(t3::srv(app, port));
  alive.run().await?;
  OK
}

#[tokio::main]
async fn main() -> Result<()> {
  #[cfg(feature = "tokio_console")]
  {
    console_subscriber::init();
  }
  #[cfg(not(feature = "tokio_console"))]
  {
    loginit::init();
  }

  if let Some((m, mut cmd)) = cmdv!(
    arg!(-c --config <path> "config dir"),
    arg!(-p --port <port> "port"),
  ) {
    if let Some(config_dir) = m.get_one::<String>("config")
      && let Some(port) = m.get_one::<String>("port")
    {
      return run(config_dir, port.parse()?).await;
    } else {
      cmd.print_help()?;
    }
  }
  OK
}
