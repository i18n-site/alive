use std::{collections::HashMap, rc::Rc};

pub use alive_api::Task;
use aok::Result;

use crate::conf::Vps;

pub struct Cluster<Conf> {
  pub name: String,
  pub vps_li: Rc<HashMap<String, Vps>>,
  pub conf: Conf,
}

pub fn load<
  TagLi: Into<Box<[String]>>,
  Arg,
  Conf: Clone,
  TaskIter: IntoIterator<Item = (Arg, TagLi)>,
>(
  wc: &crate::Conf,
  cluster_conf: HashMap<String, Conf>,
  duration: u64,
  task_load: impl Fn(Cluster<Conf>) -> TaskIter,
) -> Result<Vec<Task<Arg>>> {
  let mut r = Vec::new();
  for (cluster_name_li, conf) in cluster_conf {
    for cluster_name in cluster_name_li.split('|') {
      let vps_li = wc.cluster(cluster_name)?;
      for (arg, tag_li) in task_load(Cluster {
        name: cluster_name.into(),
        vps_li,
        conf: conf.clone(),
      }) {
        r.push(Task::new(cluster_name, arg, tag_li, duration));
      }
    }
  }
  Ok(r)
}

#[macro_export]
macro_rules! yml_cluster_load {
  ($duration:expr,$task_load:expr) => {
    pub async fn load(wc: &alive_watch::Conf) -> Result<Vec<$crate::yml_cluster::Task>> {
      $crate::yml_cluster::load(wc, alive_watch::yml!(wc), $duration, $task_load)
    }
  };
}
