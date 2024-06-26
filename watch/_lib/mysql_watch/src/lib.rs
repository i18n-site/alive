use std::collections::HashMap;

pub use alive_api::Task;
use aok::Result;
use mysql_async::OptsBuilder;
pub use mysql_async::{
  prelude::{Query, Queryable},
  Conn, Row,
};

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Conf {
  pub port: u16,
  pub password: String,
  pub user: String,
}

pub type Arg = alive_watch::yml_cluster_vps_li::Arg<Conf>;

#[macro_export]
macro_rules! load {
  ($duration: expr) => {
    pub async fn load(wc: &alive_watch::Conf) -> Result<Vec<$crate::Task<$crate::Arg>>> {
      $crate::load(wc, alive_watch::yml!(wc), $duration).await
    }
  };
  () => {
    $crate::load!(60);
  };
}

pub async fn load(
  wc: &alive_watch::Conf,
  cluster_conf: HashMap<String, Conf>,
  duration: u64,
) -> Result<Vec<Task<Arg>>> {
  alive_watch::yml_cluster_vps_li::ipv4::<Conf>(wc, cluster_conf, duration)
}

pub async fn conn(ip: std::net::IpAddr, conf: &Conf) -> Result<Conn> {
  let build = OptsBuilder::default()
    .ip_or_hostname(ip.to_string())
    .tcp_port(conf.port)
    .user(Some(conf.user.clone()))
    .prefer_socket(false)
    .pass(Some(conf.password.clone()));
  Ok(Conn::new(build).await?)
}
