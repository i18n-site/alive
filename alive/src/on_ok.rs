use alive_alter::{Msg, Recover};
use alive_plugin::Run;
use sts::sec;

// use crate::BEGIN_WARN_COUNT;

pub async fn on_ok(mut i: Run) -> (Run, Msg) {
  let now = sec();
  let err = i.err;
  let task = i.task.meta();
  let msg = if err > 0 {
    i.err = 0;
    let duration = now - i.pre_change;
    i.pre_change = now;
    Msg::Recover(Recover {
      first_warn: task.first_warn,
      host: task.host.clone(),
      tag_li: task.tag_li.clone(),
      watch_name: i.task.to_string(),
      err: i.err,
      duration,
    })
  } else {
    if i.pre_change == 0 {
      i.pre_change = now;
    }
    Msg::None
  };
  i.next_ping = now + task.interval;
  (i, msg)
}
