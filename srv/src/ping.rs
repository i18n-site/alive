use std::sync::atomic::Ordering::Relaxed;

use axum::{extract::State, http::StatusCode};
use re::err;
use tokio::time::{sleep, Duration};

use crate::Api;

pub async fn ping(State(api): Api) -> re::msg!() {
  let now = sts::sec();
  let pre = api.pre_check.load(Relaxed);
  let diff = if now > pre { now - pre } else { 0 };
  if diff > 100 {
    let msg = "❌ alive run expire";
    tracing::error!(msg);
    tokio::spawn(async {
      sleep(Duration::from_secs(3)).await;
      std::process::exit(1);
    });
    err(StatusCode::FAILED_DEPENDENCY, msg.to_owned())?
  }
  Ok(diff.to_string())
}
