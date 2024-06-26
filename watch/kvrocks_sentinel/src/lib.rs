use std::{
  collections::{HashMap, HashSet},
  net::IpAddr,
};

use alive_api::Task;
use aok::{throw, Result, OK};
use fred::{
  clients::SentinelClient,
  interfaces::{ClientLike, SentinelInterface},
  prelude::ReconnectPolicy,
  types::SentinelConfig,
};

const DURATION: u64 = 60;

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Conf {
  pub port: u16,
  pub password: String,
  pub user: Option<String>,
  pub watch: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct Arg {
  pub conf: Conf,
  pub slave_num: u8,
  pub ip: IpAddr,
}

pub async fn load(wc: &alive_watch::Conf) -> Result<Vec<Task<Arg>>> {
  let c: HashMap<String, Conf> = alive_watch::yml!(wc);
  let mut r = Vec::new();
  for (cluster, conf) in c {
    let cluster = wc.cluster(cluster)?;
    if !cluster.is_empty() {
      let slave_num = (cluster.len() - 1) as u8;
      for (vps, ip) in cluster.iter() {
        for i in &ip.ipv4_li {
          r.push(Task::new(
            vps.clone(),
            Arg {
              slave_num,
              ip: IpAddr::V4(*i),
              conf: conf.clone(),
            },
            [],
            DURATION,
          ))
        }
      }
    }
  }
  Ok(r)
}

pub async fn run(arg: &Arg) -> Result<()> {
  let mut conf = arg.conf.clone();
  let policy = ReconnectPolicy::new_constant(6, 1);
  let conn = SentinelClient::new(
    SentinelConfig {
      username: Some(conf.user.clone().unwrap_or("default".into())),
      password: Some(conf.password.clone()),
      port: conf.port,
      host: arg.ip.to_string(),
    },
    None,
    None,
    Some(policy),
  );
  conn.connect();
  conn.wait_for_connect().await?;

  let master: Vec<Vec<String>> = conn.masters().await?;
  for li in master {
    let map =
      HashMap::<String, String>::from_iter(li.chunks(2).map(|i| (i[0].clone(), i[1].clone())));
    if let Some(name) = map.get("name") {
      // for example: mymaster
      conf.watch.remove(name);
    }
    if let Some(n) = map.get("num-slaves") {
      let n: u8 = n.parse()?;
      if n != arg.slave_num {
        throw!("(slaves num {} != {} ", n, arg.slave_num);
      }
    }
  }

  let watch = conf.watch;
  if !watch.is_empty() {
    let name_li = watch.into_iter().collect::<Vec<_>>().join(",");
    throw!("{} not watch", name_li);
  }
  OK
}
