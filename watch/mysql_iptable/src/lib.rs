use std::time::Duration;

use alive_watch::{cluster_alive::ClusterAlive, yml_cluster_vps_li::Vps};
use aok::{throw, Null, Result, OK};
pub use mysql_watch::{Arg, Conf, Conn, Query, Row};
use tokio::time::timeout;

mysql_watch::load!();

#[derive(Debug, Default)]
pub struct MysqlState {
  pub server_id: String,
}

struct Alive;

const TIMEOUT: Duration = Duration::from_secs(20);

impl ClusterAlive<Conf> for Alive {
  type State = MysqlState;
  type Conn = Conn;

  async fn conn(ip: std::net::IpAddr, conf: &Conf) -> Result<mysql_watch::Conn> {
    timeout(TIMEOUT, mysql_watch::conn(ip, conf)).await?
  }

  async fn ping(mut conn: Conn) -> Result<MysqlState> {
    let server_id = if let Some(r) = "SHOW VARIABLES LIKE 'server_id'"
      .first::<(Vec<u8>, Vec<u8>), _>(&mut conn)
      .await?
    {
      let server_id = String::from_utf8_lossy(&r.1).to_string();
      if server_id.is_empty() {
        throw!("server_id is empty");
      }
      server_id
    } else {
      throw!("miss server_id");
    };

    if r"SHOW slave STATUS"
      .first::<Row, _>(&mut conn)
      .await?
      .is_some()
    {
      throw!("not master");
    };
    Ok(MysqlState { server_id })
  }

  fn check(li: &[(String, Self::State)], _conf: &Conf, _vps_li: &[Vps]) -> Null {
    if li.len() > 1 {
      let li0 = &li[0];
      let vps0 = &li0.0;
      let server_id = &li0.1.server_id;
      for (vps, i) in &li[1..] {
        if &i.server_id != server_id {
          throw!(
            "master not same : {} server_id {} != {} server_id {}",
            vps0,
            server_id,
            vps,
            i.server_id
          );
        }
      }
    }

    OK
  }
}

pub async fn run(arg: &Arg) -> Null {
  alive_watch::cluster_alive::run::<Alive, _>(arg).await
}
