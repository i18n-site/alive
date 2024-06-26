use std::cmp::min;

use alive_alter::{Msg, Warn};
use alive_plugin::Run;
use sts::sec;
// pub const DAY_MINUTE: u64 = 24 * 60;

pub async fn on_err(mut i: Run, error: aok::Error) -> (Run, Msg) {
  i.err += 1;
  let err = i.err;
  let now = sec();

  let duration = if i.err == 1 {
    i.pre_change = now;
    0
  } else {
    now - i.pre_change
  };

  i.next_ping = now + min(60, err);
  let task = i.task.meta();
  let msg = Msg::Warn(Warn {
    first_warn: task.first_warn,
    host: task.host.clone(),
    tag_li: task.tag_li.clone(),
    watch_name: i.task.to_string(),
    duration,
    err: error,
    times: err,
  });
  (i, msg)
}
