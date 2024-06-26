use std::{collections::HashMap, net::IpAddr};

use alive_api::Task;
use aok::{ensure, Result, OK};

#[derive(Debug)]
pub struct Arg {
  pub ip: IpAddr,
  pub port: u16,
  pub ping: String,
}

const DURATION: u64 = 60;

pub async fn load(wc: &alive_watch::Conf) -> Result<Vec<Task<Arg>>> {
  let c: HashMap<String, HashMap<u16, Vec<String>>> = alive_watch::yml!(wc);
  let mut r = Vec::new();
  for (cluster, port_ping_li) in c {
    for (host, ip) in wc.cluster(cluster)?.iter() {
      for ip in &ip.ipv4_li {
        for (port, ping_li) in &port_ping_li {
          for ping in ping_li {
            r.push(Task::new(
              host.clone(),
              Arg {
                ip: IpAddr::V4(*ip),
                port: *port,
                ping: ping.clone(),
              },
              [ping.clone()],
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
  let ping = &*arg.ping;
  let r = ireq::get(format!("http://{}:{}/ping{}", arg.ip, arg.port, ping)).await?;
  ensure!(r == ping, "{} != {}", r, ping);
  OK
}
