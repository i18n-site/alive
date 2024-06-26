use std::{collections::HashMap, net::IpAddr};

use alive_api::Task;
use aok::Result;

#[derive(Debug)]
pub struct Vps {
  pub ip: IpAddr,
  pub name: String,
}

#[derive(Debug)]
pub struct Arg<Conf> {
  pub vps_li: Vec<Vps>,
  pub conf: Conf,
}

pub mod li {
  use super::Vps;
  pub fn ipv4(name: &str, vps: &crate::conf::Vps) -> Vec<Vps> {
    vps
      .ipv4_li
      .iter()
      .map(|ip| Vps {
        name: name.into(),
        ip: (*ip).into(),
      })
      .collect::<Vec<_>>()
  }
}

pub fn load<Conf: Clone>(
  wc: &crate::Conf,
  cluster_conf: HashMap<String, Conf>,
  duration: u64,
  ip_li: impl Fn(&str, &crate::conf::Vps) -> Vec<Vps>,
) -> Result<Vec<Task<Arg<Conf>>>> {
  crate::yml_cluster::load(wc, cluster_conf, duration, |cluster| {
    let arg = Arg::<Conf> {
      conf: cluster.conf,
      vps_li: cluster
        .vps_li
        .iter()
        .map(|(name, vps)| ip_li(name, vps))
        .collect::<Vec<_>>()
        .into_iter()
        .flatten()
        .collect(),
    };
    [(arg, [])]
  })
}

pub fn ipv4<Conf: Clone>(
  wc: &crate::Conf,
  cluster_conf: HashMap<String, Conf>,
  duration: u64,
) -> Result<Vec<Task<Arg<Conf>>>> {
  load(wc, cluster_conf, duration, li::ipv4)
}

#[macro_export]
macro_rules! yml_cluster_vps_li_ipv4_load {
  ($conf:ty, $duration:expr) => {
    pub async fn load(
      wc: &alive_watch::Conf,
    ) -> Result<Vec<$crate::yml_cluster::Task<$crate::yml_cluster_vps_li::Arg<Conf>>>> {
      $crate::yml_cluster_vps_li::ipv4::<$conf>(wc, alive_watch::yml!(wc), $duration)
    }
  };
}
