use aok::{throw, Null, Result};
use tokio::task::JoinSet;

use crate::yml_cluster_vps_li::{Arg, Vps};

pub trait ClusterAlive<Conf> {
  type State;
  type Conn: Send;

  fn conn(
    ip: std::net::IpAddr,
    conf: &Conf,
  ) -> impl std::future::Future<Output = Result<Self::Conn>> + Send;

  fn ping(conn: Self::Conn) -> impl std::future::Future<Output = Result<Self::State>> + Send;
  fn check(li: &[(String, Self::State)], conf: &Conf, vps_li: &[Vps]) -> Null;
}

pub async fn run<A: ClusterAlive<Conf>, Conf>(arg: &Arg<Conf>) -> Null
where
  for<'a> <A as ClusterAlive<Conf>>::State: Send + 'a,
  <A as ClusterAlive<Conf>>::State: Send,
  Conf: Send + Sync + 'static + Clone,
{
  let mut ing = JoinSet::new();
  let vps_len = arg.vps_li.len();
  let conf = &arg.conf;
  let mut rli = Vec::with_capacity(vps_len);

  for vps in &arg.vps_li {
    let vps_name = vps.name.clone();
    let ip = vps.ip;
    let conf = conf.clone();
    ing.spawn(async move {
      match A::conn(ip, &conf).await {
        Ok(conn) => (vps_name, A::ping(conn).await),
        Err(e) => (vps_name, Err(e)),
      }
    });
  }

  while let Some(r) = ing.join_next().await {
    let (vps, r) = r?;
    rli.push((
      vps.clone(),
      throw!(
        r; "{}",  vps
      ),
    ))
  }

  A::check(&rli, conf, &arg.vps_li[..])
}
