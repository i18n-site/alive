use std::collections::HashMap;

use alive_api::Task;
use alive_watch::Conf;
use aok::{ensure, Result, OK};

#[derive(Debug)]
pub struct Arg {
  pub ip: u8,
  pub url: String,
  pub host: String,
}

genv::s!(CNAME_URL);

const DURATION: u64 = 10 * 60;

pub async fn load(wc: &Conf) -> Result<Vec<Task<Arg>>> {
  let conf: HashMap<String, String> = alive_watch::yml!(wc);
  let mut r = Vec::with_capacity(conf.len() * 2);
  for (host, url) in conf {
    for ip in [4, 6] {
      r.push(Task::new_with_first_warn(
        host.clone(),
        Arg {
          ip,
          host: host.clone(),
          url: url.clone(),
        },
        [format!("IPV{ip}")],
        DURATION,
        30,
      ));
    }
  }
  Ok(r)
}

const MAX_TRY: usize = 3;

pub async fn run(arg: &Arg) -> Result<()> {
  let ip = arg.ip;
  let host = &arg.host;
  let url = format!("https://{}/{}/{}/{}", &*CNAME_URL, ip, host, arg.url);
  for i in 0..=MAX_TRY {
    match ireq::get(&url).await {
      Ok(txt) => {
        ensure!(txt.starts_with("["), "cname_flatten {host} ipv{ip} : {txt}");
        return OK;
      }
      Err(e) => {
        if i == MAX_TRY {
          return Err(e);
        }
        tracing::error!("{host} {ip}: {}", e);
      }
    };
  }
  OK
}
