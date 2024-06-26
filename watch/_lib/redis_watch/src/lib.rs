use std::net::SocketAddr;

use alive_watch::yml_cluster_vps_li::Arg as _Arg;
use aok::Result;
use fred::{
  clients::RedisClient,
  interfaces::ClientLike,
  prelude::{ReconnectPolicy, RedisConfig, ServerConfig},
  types::Server,
};

pub const DURATION: u64 = 60;
pub type Arg = _Arg<Conf>;

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Conf {
  pub port: u16,
  pub password: String,
  pub user: Option<String>,
}

#[derive(Debug)]
pub enum State {
  SlaveLi(Vec<SocketAddr>),
  Master(SocketAddr),
}

pub async fn conn(ip: std::net::IpAddr, conf: &Conf) -> Result<RedisClient> {
  let conf = RedisConfig {
    version: fred::types::RespVersion::RESP3,
    server: ServerConfig::Centralized {
      server: Server {
        port: conf.port,
        host: ip.to_string().into(),
      },
    },
    username: Some(conf.user.clone().unwrap_or("default".into())),
    password: Some(conf.password.clone()),
    ..Default::default()
  };

  /*
  https://docs.rs/fred/latest/fred/types/enum.ReconnectPolicy.html#method.new_constant
  */
  let policy = ReconnectPolicy::new_constant(6, 1);
  let client = RedisClient::new(conf, None, None, Some(policy));
  client.connect();
  client.wait_for_connect().await?;
  Ok(client)
}
