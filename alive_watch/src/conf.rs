use std::{
  collections::HashMap,
  net::{Ipv4Addr, Ipv6Addr},
  path::{Path, PathBuf},
  rc::Rc,
};

use aok::Result;
use dashmap::DashMap;
use tracing::warn;
#[derive(Debug)]
pub struct Vps {
  pub ipv4_li: Vec<Ipv4Addr>,
  pub ipv6_li: Vec<Ipv6Addr>,
}

#[derive(Debug)]
pub struct Conf {
  pub root: PathBuf,
  pub cluster: DashMap<String, Rc<HashMap<String, Vps>>>,
}

impl Conf {
  #[cfg(feature = "yml")]
  pub fn yml<T: serde::de::DeserializeOwned>(&self, file: &str) -> Result<T> {
    let fp = self.join(file);
    let yml = ifs::r(fp)?;
    if yml.is_empty() {
      warn!("{file} IS EMPTY");
    }
    Ok(serde_yaml::from_slice(&yml)?)
  }

  pub fn new(root: impl Into<PathBuf>) -> Self {
    Self {
      root: root.into(),
      cluster: Default::default(),
    }
  }

  pub fn cluster(&self, name: impl AsRef<str>) -> Result<Rc<HashMap<String, Vps>>> {
    let name = name.as_ref();
    let r = if let Some(r) = self.cluster.get(name) {
      r
    } else {
      let fp = self.root.join(format!("cluster/{name}.yml"));
      let yml = ifs::r(fp)?;
      if yml.is_empty() {
        warn!("CLUSTER {name} IS EMPTY");
      }
      let m: HashMap<String, Vec<String>> = serde_yaml::from_slice(&yml)?;
      let mut cluster = HashMap::with_capacity(m.len());
      for (name, ip_li) in m {
        let len = ip_li.len() / 2;
        let mut ipv4_li = Vec::with_capacity(len);
        let mut ipv6_li = Vec::with_capacity(len);
        for ip in ip_li {
          if ip.contains(':') {
            ipv6_li.push(ip.parse()?);
          } else {
            ipv4_li.push(ip.parse()?);
          }
        }
        cluster.insert(name, Vps { ipv4_li, ipv6_li });
      }

      self.cluster.insert(name.into(), cluster.into());
      self.cluster.get(name).unwrap()
    }
    .value()
    .clone();
    Ok(r)
  }

  pub fn join(&self, path: impl AsRef<Path>) -> PathBuf {
    self.root.join("watch").join(path)
  }
}
