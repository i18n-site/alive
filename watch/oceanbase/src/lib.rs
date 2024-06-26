use std::collections::HashSet;

use alive_watch::{cluster_alive::ClusterAlive, yml_cluster_vps_li::Vps};
use aok::{throw, Null, Result, OK};
use mysql_watch::Conf;
pub use mysql_watch::{Arg, Conn, Query, Row};

mysql_watch::load!();

#[derive(Debug, Default)]
pub struct MysqlState {
  pub li: Vec<(String, String)>,
}

struct Alive;

impl ClusterAlive<Conf> for Alive {
  type State = MysqlState;
  type Conn = Conn;

  async fn conn(ip: std::net::IpAddr, conf: &Conf) -> Result<mysql_watch::Conn> {
    mysql_watch::conn(ip, conf).await
  }

  async fn ping(mut conn: Conn) -> Result<MysqlState> {
    let li: Vec<(String, String)> = "SELECT ZONE,STATUS FROM oceanbase.DBA_OB_SERVERS"
      .fetch(&mut conn)
      .await?;
    let state = MysqlState {
      li: li.into_iter().filter(|i| i.1 != "ACTIVE").collect(),
    };
    Ok(state)
  }

  fn check(li: &[(String, Self::State)], _conf: &Conf, _vps_li: &[Vps]) -> Null {
    let mut err = vec![];
    let mut exist = HashSet::new();
    for (_srv, state) in li {
      for (vps, state) in state.li.iter() {
        if !exist.contains(vps) {
          err.push(format!("{} {}", vps, state));
          exist.insert(vps);
        }
      }
    }
    if !err.is_empty() {
      throw!("{}", err.join(";"));
    }
    OK
  }
}

pub async fn run(arg: &Arg) -> Null {
  alive_watch::cluster_alive::run::<Alive, _>(arg).await
}
