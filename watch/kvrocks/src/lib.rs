use std::{
  collections::HashMap,
  net::{IpAddr, SocketAddr},
};

use alive_watch::{
  cluster_alive::ClusterAlive, yml_cluster_vps_li::Vps, yml_cluster_vps_li_ipv4_load,
};
use aok::{throw, Null, Result, OK};
use fred::{clients::RedisClient, interfaces::ClientLike};
pub use redis_watch::Arg;
use redis_watch::{conn, Conf, State, DURATION};

yml_cluster_vps_li_ipv4_load!(Conf, DURATION);

struct Alive;

impl ClusterAlive<Conf> for Alive {
  type State = State;
  type Conn = RedisClient;

  async fn conn(ip: std::net::IpAddr, conf: &Conf) -> Result<<Self as ClusterAlive<Conf>>::Conn> {
    conn(ip, conf).await
  }

  async fn ping(conn: Self::Conn) -> Result<Self::State> {
    let r: String = conn.info(Some(fred::types::InfoKind::Replication)).await?;
    if let Some(p) = r.find("role:") {
      let r = &r[p + 5..];
      let li = r.split("\r\n");
      if r.starts_with("master") {
        let mut slave_li = Vec::with_capacity(2);
        for i in li {
          if let Some(i) = i.strip_prefix("slave") {
            if let Some(p) = i.find(":ip=") {
              let mut iter = i[p + 4..].split(',');
              if let Some(ip) = iter.next() {
                if let Some(port) = iter.next() {
                  if port.len() > 5 {
                    slave_li.push(SocketAddr::new(ip.parse()?, port[5..].parse()?));
                  }
                }
              }
            }
          }
        }
        return Ok(Self::State::SlaveLi(slave_li));
      } else if r.starts_with("slave") {
        let mut host = "";
        let mut port = "";

        for i in li {
          if i.starts_with("master_link_status") {
            if !i.ends_with(":up") {
              throw!("{}", i);
            }
          } else if let Some(h) = i.strip_prefix("master_host:") {
            host = h;
          } else if let Some(p) = i.strip_prefix("master_port:") {
            port = p;
          }
        }
        return Ok(Self::State::Master(SocketAddr::new(
          host.parse()?,
          port.parse()?,
        )));
      }
    }
    throw!("NO INFO REPLICATION");
  }

  fn check(li: &[(String, Self::State)], conf: &Conf, vps_li: &[Vps]) -> Null {
    let mut master = None;
    let mut master_ip = None;
    let mut slave_count = 0;
    let mut vps_map: HashMap<IpAddr, std::string::String> =
      HashMap::from_iter(vps_li.iter().map(|i| (i.ip, i.name.clone())));

    for (name, state) in li {
      match state {
        State::Master(master) => {
          slave_count += 1;
          let port = master.port();
          if port != conf.port {
            throw!(
              "slave {} master.port {} != conf.port {}",
              name,
              port,
              conf.port
            );
          }
          let ip = master.ip();
          if let Some(master_ip) = master_ip {
            if master_ip != ip {
              throw!("slave {} master_ip not same", name);
            }
          } else {
            master_ip = Some(ip);
            vps_map.remove(&ip);
          }
        }
        State::SlaveLi(slave_li) => {
          if let Some(master) = master {
            throw!("two master_count : {} & {}", name, master);
          }
          master = Some(name);

          for i in slave_li {
            if i.port() == conf.port {
              let ip = i.ip();
              vps_map.remove(&ip);
            }
          }
        }
      }
    }

    if let Some(master) = master {
      if slave_count == 0 {
        throw!("master no slave");
      }
      if let Some(vps) = vps_map.keys().next() {
        throw!("{} not slave of master {}", vps, master);
      }
    } else {
      throw!("no master");
    }

    OK
  }
}

pub async fn run(arg: &Arg) -> Null {
  alive_watch::cluster_alive::run::<Alive, _>(arg).await
}
