use std::{collections::HashMap, net::IpAddr};

use alive_api::Task;
use aok::{throw, Result, OK};
use tokio::task::JoinSet;

#[derive(Debug)]
pub struct Arg {
  pub port: u16,
  pub srv: String,
  pub vps_li: Vec<(String, IpAddr)>,
}

const DURATION: u64 = 60;

pub async fn load(wc: &alive_watch::Conf) -> Result<Vec<Task<Arg>>> {
  let conf: HashMap<String, HashMap<String, u16>> = alive_watch::yml!(wc);
  let mut r = Vec::new();
  for (cluster_name, srv_port) in conf {
    for (srv, port) in srv_port {
      let mut vps_li = Vec::new();
      for (name, ip) in wc.cluster(&cluster_name)?.iter() {
        for ip in &ip.ipv4_li {
          vps_li.push((name.clone(), (*ip).into()));
        }
      }
      r.push(Task::new(
        srv.clone(),
        Arg { vps_li, port, srv },
        [],
        DURATION,
      ));
    }
  }
  Ok(r)
}

pub async fn run(arg: &Arg) -> Result<()> {
  let port = arg.port;
  let mut ing = JoinSet::new();
  for (name, ip) in &arg.vps_li {
    let name = name.clone();
    let url = format!("http://{}:{}/ping/ver", ip, port);
    ing.spawn(async move { (name, ireq::get(url).await) });
  }

  let mut map = HashMap::new();

  let srv = arg.srv.clone();
  let prefix = format!("{} ", srv);
  while let Some(r) = ing.join_next().await {
    let (vps, req) = r?;
    let ver = throw!(req; "{}", vps);
    if let Some(ver) = ver.strip_prefix(&prefix) {
      map.insert(ver.to_owned(), vps);
    } else {
      throw!("'{}' 不以 {} 开头", ver, srv);
    }
  }

  if map.len() > 1 {
    let mut li = Vec::new();
    for (ver, vps) in map {
      li.push(format!("{} {}", vps, ver));
    }
    throw!("版本不一致 : {}", li.join(" , "));
  }

  OK
}
