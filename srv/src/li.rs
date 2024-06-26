use alive_api as api;
use axum::extract::State;

use crate::Api;

pub async fn post(State(api): Api) -> re::msg!() {
  Ok::<api::Li, _>(api.proto())
}
