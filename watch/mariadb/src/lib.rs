// use std::collections::HashSet;

use alive_watch::{cluster_alive::ClusterAlive, yml_cluster_vps_li::Vps};
use aok::{throw, Null, Result, OK};
use mysql_watch::Conf;
pub use mysql_watch::{Arg, Conn, Query, Row};

mysql_watch::load!();

#[derive(Debug, Default)]
pub struct MysqlState {
  pub is_master: bool,
}

struct Alive;

impl ClusterAlive<Conf> for Alive {
  type State = MysqlState;
  type Conn = Conn;

  async fn conn(ip: std::net::IpAddr, conf: &Conf) -> Result<mysql_watch::Conn> {
    mysql_watch::conn(ip, conf).await
  }

  async fn ping(mut conn: Conn) -> Result<MysqlState> {
    let is_master: bool = "SELECT CASE WHEN VARIABLE_VALUE=0 THEN TRUE ELSE FALSE END AS IS_MASTER FROM information_schema.GLOBAL_STATUS WHERE VARIABLE_NAME='SLAVES_RUNNING'"
      .first(&mut conn)
      .await?.unwrap();
    Ok(MysqlState { is_master })
  }

  fn check(li: &[(String, Self::State)], _conf: &Conf, _vps_li: &[Vps]) -> Null {
    let mut master = 0;
    for i in li {
      if i.1.is_master {
        master += 1;
      }
    }
    if master != 1 {
      throw!("{master} master");
    }
    OK
  }
}

pub async fn run(arg: &Arg) -> Null {
  alive_watch::cluster_alive::run::<Alive, _>(arg).await
}
