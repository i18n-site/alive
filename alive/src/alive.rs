use std::{
  path::Path,
  sync::{atomic::Ordering::Relaxed, Arc},
};

pub use alive_alter::Alter;
use alive_alter::{Load, Msg};
pub use alive_plugin::{Run, Watch};
use alive_watch::Conf;
use aok::Result;
use chrono::Local;
use skiplist::ordered_skiplist::OrderedSkipList;
use tokio::{
  task::JoinSet,
  time::{sleep, timeout, Duration},
};
use tracing::info;

use crate::{on_err, on_ok, Api};

pub const MINUTE: Duration = Duration::from_secs(60);

#[derive(Debug)]
pub struct Alive<A: Alter + Load> {
  pub run: OrderedSkipList<Run>,
  pub alter: A,
  pub api: Arc<Api>,
  pub conf: Conf,
}

impl<A: Alter + Load> Alive<A> {
  pub async fn load(dir: &Path) -> Result<Self> {
    let api = Api::new();
    let conf = Conf::new(dir);
    let run = Watch::load(&conf).await?;
    for i in &run {
      api.touch(i);
    }
    Ok(Self {
      api: api.into(),
      run,
      alter: A::load(&dir.join("alter")).await?,
      conf,
    })
  }

  pub async fn run(&mut self) -> Result<()> {
    let mut ing = JoinSet::new();
    loop {
      let now = sts::ms();
      let now_sec = now / 1000;
      self.api.pre_check.store(now_sec, Relaxed);
      while let Some(i) = self.run.pop_front() {
        if i.next_ping * 1000 > now {
          self.run.insert(i);
          break;
        }
        ing.spawn(async move {
          let task = &i.task.meta();
          info!("{} {}", i.task, task.host);
          match timeout(MINUTE, Watch::run(&i.task)).await {
            Ok(r) => match r {
              Ok(()) => on_ok(i).await,
              Err(error) => on_err(i, error).await,
            },
            Err(error) => on_err(i, error.into()).await,
          }
        });
      }

      if !ing.is_empty() {
        while let Some(i) = ing.join_next().await {
          if let Ok((mut i, msg)) = xerr::ok!(i) {
            let cost = sts::ms() - now;
            i.cost_li.push(cost as u32);
            let task = i.task.meta();
            let host = &task.host;
            info!("{} {} cost {}", i.task, host, cost);
            {
              let t = self.api.m.get(&i.task.to_string()).unwrap();
              let t = t.get(host).unwrap();
              let mut t = t.get_mut(&i.id).unwrap();
              if let Msg::Warn(ref msg) = msg {
                t.1 = Some(msg.err.to_string())
              } else {
                t.1 = None;
                let s = &mut t.0;
                s.cost_sum += cost;
                s.runed += 1;
                s.avg10 = i.cost_li.iter().sum::<u32>() / (i.cost_li.len() as u32);
              };
              // 0 的时候返回 NaNd, 应该是前端protobuf解析库的问题
              t.0.duration = std::cmp::max(1, std::cmp::max(now_sec, i.pre_change) - i.pre_change);
            }
            self.run.insert(i);
            match msg {
              Msg::None => {}
              Msg::Recover(r) => {
                self.alter.recover(&r).await?;
              }
              Msg::Warn(r) => {
                self.alter.warn(&r).await?;
              }
            };
          }
        }
        info!("{}", Local::now().format("%Y-%m-%d %H:%M:%S"));
      }
      sleep(Duration::from_secs(1)).await;
    }
  }
}
