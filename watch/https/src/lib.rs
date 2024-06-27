use std::{collections::HashMap, net::IpAddr};

use alive_api::Task;
use aok::{Result, OK};
use tls_ping::tls_ping;

#[derive(serde::Deserialize, Debug)]
pub struct HostIp {
  v4: Option<Vec<String>>,
  v6: Option<Vec<String>>,
  v4v6: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Conf {
  cluster: HashMap<String, HostIp>,
  host: HashMap<String, HashMap<String, Vec<String>>>,
}

#[derive(Debug)]
pub struct Arg {
  pub ip: IpAddr,
  pub host: String,
}

const DURATION: u64 = 60;
const TIMEOUT: u64 = 30;

pub async fn load(wc: &alive_watch::Conf) -> Result<Vec<Task<Arg>>> {
  let c: Conf = alive_watch::yml!(wc);
  let mut r = Vec::new();
  for (cluster, host_ip) in c.cluster {
    for (hostname, ip) in wc.cluster(cluster)?.iter() {
      macro_rules! push {
        ($attr:ident, $ipv:ident, $v:ident, $tag:ident) => {
          for i in &ip.$attr {
            if let Some(li) = &host_ip.$ipv {
              for host in li {
                r.push(Task::new(
                  host.clone(),
                  Arg {
                    ip: IpAddr::$v(*i),
                    host: host.into(),
                  },
                  [hostname.into(), stringify!($tag).into()],
                  DURATION,
                ))
              }
            }
          }
        };
      }
      push!(ipv4_li, v4, V4, IPV4);
      push!(ipv6_li, v6, V6, IPV6);
      push!(ipv4_li, v4v6, V4, IPV4);
      push!(ipv6_li, v4v6, V6, IPV6);
    }
  }
  for (cluster, vps_host_map) in &c.host {
    for (vps, ip) in wc.cluster(cluster)?.iter() {
      if let Some(host_li) = vps_host_map.get(vps) {
        for host in host_li {
          for ip in &ip.ipv4_li {
            r.push(Task::new(
              &host,
              Arg {
                ip: IpAddr::V4(*ip),
                host: host.into(),
              },
              [vps.into()],
              DURATION,
            ))
          }
        }
      }
    }
  }
  Ok(r)
}

pub async fn run(arg: &Arg) -> Result<()> {
  tls_ping(&arg.host, arg.ip, TIMEOUT).await?;
  OK
}
