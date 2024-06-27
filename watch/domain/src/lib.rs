use std::{
  collections::HashSet,
  time::{SystemTime, UNIX_EPOCH},
};

use alive_api::Task;
use aok::{throw, Result, OK};
use mysql_async::{
  prelude::{Query, WithParams},
  Conn, Opts, OptsBuilder, SslOpts,
};

mod whois;
use crate::whois::{Domain, Whois};

const DURATION: u64 = 86400;
const DAY: u64 = 86400;
const Q_SEC: u64 = DAY * 90;

#[derive(serde::Deserialize, Debug)]
pub struct Conf {
  token: String,
  li: HashSet<String>,
  mysql: String,
}

#[derive(Debug)]
pub struct Arg {
  whois: Whois,
  li: HashSet<String>,
  mysql: Opts,
  q: String,
}

pub async fn load(wc: &alive_watch::Conf) -> Result<Vec<Task<Arg>>> {
  let c: Conf = alive_watch::yml!(wc);
  let opts = Opts::from_url(&format!("mysql://{}", c.mysql))?;
  let opts = OptsBuilder::from_opts(opts);
  let opts = opts.ssl_opts(SslOpts::default().with_danger_accept_invalid_certs(true));
  let opts: Opts = opts.into();

  let mut conn = Conn::new(opts.clone()).await?;
  r#"CREATE TABLE IF NOT EXISTS domain (id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY, domain VARCHAR(255) NOT NULL UNIQUE, expire BIGINT UNSIGNED NOT NULL, pre_check_ts BIGINT UNSIGNED NOT NULL)"#.ignore(&mut conn).await?;

  let li = &c.li;

  let mut q = Vec::with_capacity(li.len() + 2);
  if !c.li.is_empty() {
    q.push("SELECT domain,expire,pre_check_ts FROM domain WHERE domain IN (".to_owned());
    for i in c.li.iter() {
      q.push(sonic_rs::to_string(i)?);
      q.push(",".into());
    }
    q.pop();
    q.push(")".into());
  }

  Ok(vec![Task::new_with_first_warn(
    format!("{} 域名", c.li.len()),
    Arg {
      q: q.join(""),
      whois: Whois::new(&c.token),
      li: c.li,
      mysql: opts,
    },
    [],
    DURATION,
    0,
  )])
}

async fn upsert(
  conn: &mut Conn,
  domain: &str,
  expire: u64,
  pre_check_ts: u64,
) -> Result<(), mysql_async::Error> {
  "INSERT INTO domain (domain,expire,pre_check_ts)VALUES(?,?,?) ON DUPLICATE KEY UPDATE expire=VALUES(expire),pre_check_ts=VALUES(pre_check_ts)".with(
    (domain, expire, pre_check_ts)
  ).ignore(conn).await?;
  Ok(())
}

pub async fn run(arg: &Arg) -> Result<()> {
  if arg.q.is_empty() {
    return Ok(());
  }
  let mut conn = Conn::new(arg.mysql.clone()).await?;
  let rows: Vec<(String, u64, u64)> = arg.q.clone().fetch(&mut conn).await?;
  let mut host_set = arg.li.clone();
  let mut warn = vec![];
  let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

  macro_rules! upsert {
    ($domain:expr) => {{
      let domain = &$domain;
      let d = Domain::new(domain);
      let expire = d.expire(&arg.whois).await?;
      upsert(&mut conn, &domain, expire, now).await?;
      expire
    }};
  }

  for (domain, mut expire, pre_check_ts) in rows {
    if (now + Q_SEC) > expire {
      let d = Domain::new(&domain);

      let duration = if d.free { 0 } else { DAY };
      if pre_check_ts + duration < now {
        expire = upsert!(domain);
      }

      if expire < now {
        warn.push(format!("{} 过期了", domain));
      } else {
        let day = (expire - now) / DAY;
        warn.push(format!("{} 还有 {} 天过期", domain, day));
      }
    }
    // else {
    //   let day = (expire - now) / DAY;
    // }

    host_set.remove(&domain);
  }
  if !host_set.is_empty() {
    for domain in host_set {
      upsert!(domain);
    }
  }
  // let proxy = format!("{}:{}", arg.ip, &*IPV6_PROXY_PORT);
  // let url = &*IPV6_PROXY_TEST_URL;
  // let r = preq1::post_form(0, &[preq1::proxy(&proxy)], url, [("q", "I")]).await?;
  // let result = &*IPV6_PROXY_TEST_RESULT;
  // ensure!(
  //   r == result,
  //   format!(
  //     "ipv6_proxy {} result mismatch : {} != {}",
  //     proxy,
  //     String::from_utf8_lossy(&r),
  //     result
  //   )
  // );

  if !warn.is_empty() {
    let warn = warn.join("\n");
    throw!(warn);
  }
  OK
}
