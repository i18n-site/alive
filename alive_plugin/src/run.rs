use std::sync::atomic::{AtomicU64, Ordering};

use circular_queue::CircularQueue;

use crate::watch::EnumTask;

pub static ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct Run {
  pub task: EnumTask,
  pub id: u64,
  pub next_ping: u64,
  pub err: u64,
  pub pre_change: u64,
  pub cost_li: CircularQueue<u32>,
}

impl Run {
  pub fn new(task: EnumTask) -> Self {
    Self {
      id: ID.fetch_add(1, Ordering::Relaxed),
      task,
      next_ping: 0,
      err: 0,
      pre_change: 0,
      cost_li: CircularQueue::with_capacity(10),
    }
  }
}

impl PartialEq for Run {
  fn eq(&self, other: &Self) -> bool {
    self.next_ping == other.next_ping
  }
}

impl PartialOrd for Run {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.next_ping.cmp(&other.next_ping))
  }
}
