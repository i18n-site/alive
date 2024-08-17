use std::collections::HashMap;

use alive_api::Task;
use aok::{throw, Result, OK};
use chrono::{Local, TimeZone};

#[derive(serde::Deserialize, Debug)]
pub struct Conf(pub HashMap<String, String>);

#[derive(Debug)]
pub struct Arg {
  pub cdn: String,
  pub db: String,
}

const DURATION: u64 = 60;

pub async fn load(wc: &alive_watch::Conf) -> Result<Vec<Task<Arg>>> {
  let c: Conf = alive_watch::yml!(wc);
  let mut r = Vec::new();
  for (cdn, db) in c.0 {
    r.push(Task::new(cdn.clone(), Arg { cdn, db }, [], DURATION));
  }
  Ok(r)
}

pub async fn run(arg: &Arg) -> Result<()> {
  let tls = tokio_postgres_tls::MakeRustlsConnect {
    config: tlsinit::CLIENT.clone(),
  };

  let cdn = arg.cdn.clone();
  let (pg, conn) = tokio_postgres::connect(&arg.db, tls).await?;
  tokio::spawn(async move {
    if let Err(e) = conn.await {
      eprintln!("{cdn} : {e}");
    }
  });

  let li = pg
    .query("SELECT name,ts FROM cdn.vps_not_refresh()", &[])
    .await?;

  let mut err = vec![];
  for row in li {
    let name: String = row.get("name");
    let ts: i64 = row.get("ts");
    if let chrono::LocalResult::Single(ts) = Local.timestamp_opt(ts, 0) {
      let ts = ts.format("%Y-%m-%d %H:%M:%S").to_string();
      err.push(format!("{name} 上次更新 {ts}"));
    }
  }

  if !err.is_empty() {
    throw!("{}", err.join("\n"));
  }
  OK
}
