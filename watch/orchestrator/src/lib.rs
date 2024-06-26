use std::{collections::HashMap, net::IpAddr};

use alive_api::Task;
use aok::{throw, Result, OK};
use sonic_rs::{Deserialize, Serialize};
use tokio::task::JoinSet;

const DURATION: u64 = 60;

#[derive(Clone, serde::Deserialize, Debug)]
pub struct Conf {
  pub port: u16,
  pub password: String,
  pub user: String,
}

#[derive(Debug, Clone)]
pub struct Arg {
  pub conf: Conf,
  pub ip_map: HashMap<IpAddr, String>,
}

pub async fn load(wc: &alive_watch::Conf) -> Result<Vec<Task<Arg>>> {
  let c: HashMap<String, Conf> = alive_watch::yml!(wc);
  let mut r = Vec::new();
  for (cluster_name, conf) in c {
    let cluster = wc.cluster(&cluster_name)?;

    let mut ip_map = HashMap::new();
    for (vps, ip) in cluster.iter() {
      for i in ip.ipv4_li.iter() {
        ip_map.insert(IpAddr::V4(*i), vps.clone());
      }
    }
    r.push(Task::new(
      cluster_name.clone(),
      Arg {
        conf: conf.clone(),
        ip_map,
      },
      [],
      DURATION,
    ))
    // for (vps, ip) in cluster.iter() {
    //   Task::new(
    //     vps.clone(),
    //     Arg {
    //       ip_li: ip.ipv4_li.iter().map(|i| IpAddr::V4(*i)).collect(),
    //       conf: conf.clone(),
    //     },
    //     [],
    //     DURATION,
    //   )
    // }
  }
  Ok(r)
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct HealthDetail {
  IsRaftLeader: bool,
  RaftHealthyMembers: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct Health {
  Code: String,
  Message: String,
  Details: HealthDetail,
}

pub async fn run(arg: &Arg) -> Result<()> {
  let mut ing = JoinSet::new();
  let conf = &arg.conf;
  let mut ip_map = arg.ip_map.clone();
  for (ip, name) in &arg.ip_map {
    let port = conf.port;
    let url = format!("http://{ip}:{port}/api/health");
    let password = conf.password.clone();
    let user = conf.user.clone();
    let name = name.clone();
    ing.spawn(async move {
      (
        name,
        ireq::REQ
          .get(url)
          .basic_auth(user, Some(&password))
          .send()
          .await,
      )
    });
  }
  let mut master = None;

  while let Some(i) = ing.join_next().await {
    let (name, r) = i?;
    let r = throw!(r; "{}", name);
    let r = throw!(r.text().await; "{}", name);
    let r: Health = throw!(sonic_rs::from_str(&r); "{}", name);
    if r.Code != "OK" {
      throw!("{} : {}", name, r.Message);
    }
    if r.Details.IsRaftLeader {
      if let Some(master) = master {
        throw!("two master {} & {}", name, master);
      } else {
        master = Some(name);
      }
      if let Some(mli) = r.Details.RaftHealthyMembers {
        for i in mli {
          ip_map.remove(&i.parse()?);
        }
        if !ip_map.is_empty() {
          throw!(
            "{} not health",
            ip_map
              .keys()
              .map(|i| i.to_string())
              .collect::<Vec<_>>()
              .join(" & ")
          );
        }
      }
    }
  }
  if master.is_none() {
    throw!("no master");
  }

  OK
}
