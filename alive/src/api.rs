use std::sync::{
  atomic::{AtomicU64, Ordering::Relaxed},
  Arc,
};

pub use alive_api::{Kind, Li, Site, State};
pub use alive_plugin::Run;
use dashmap::DashMap;

pub type Map = Arc<
  DashMap<
    String, // kind
    DashMap<
      Box<str>, // site.id
      DashMap<u64, (State, Option<String>)>,
    >,
  >,
>;

#[derive(Debug)]
pub struct Api {
  pub pre_check: AtomicU64,
  pub m: Map,
}

impl Default for Api {
  fn default() -> Self {
    Self::new()
  }
}

impl Api {
  pub fn new() -> Self {
    Self {
      pre_check: Default::default(),
      m: Default::default(),
    }
  }

  pub fn touch(&self, run: &Run) {
    let task = &run.task;
    let meta = task.meta();
    *self
      .m
      .entry(task.to_string())
      .or_default()
      .entry(meta.host.clone())
      .or_default()
      .entry(run.id)
      .or_default() = (
      State {
        tag_li: meta.tag_li.to_vec(),
        ..Default::default()
      },
      None,
    );
  }

  pub fn proto(&self) -> Li {
    Li {
      sec_since_pre_check: (sts::sec() - self.pre_check.load(Relaxed)) as _,
      kind_li: self
        .m
        .iter()
        .map(|i| {
          let v = i.value();
          Kind {
            name: i.key().to_string(),
            site_li: v
              .iter()
              .map(|i| {
                let k = i.key();
                let id_state = i.value();
                let mut ok_li = Vec::with_capacity(v.len());
                let mut err_li = Vec::with_capacity(0);
                for j in id_state.iter() {
                  let (state, err) = j.value().clone();
                  if let Some(err) = err {
                    err_li.push(alive_api::Err {
                      err,
                      state: Some(state),
                    });
                  } else {
                    ok_li.push(state);
                  };
                }
                Site {
                  host: k.clone().into(),
                  ok_li,
                  err_li,
                }
              })
              .collect(),
          }
        })
        .collect(),
    }
  }
}
