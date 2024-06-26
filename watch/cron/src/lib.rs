use aok::{throw, Null, Result, OK};
use mysql_watch::conn;
pub use mysql_watch::{Arg, Conn, Query, Row};

// 每小时运行一次检查
mysql_watch::load!(3600);

pub async fn run(arg: &Arg) -> Null {
  let vps_li = &arg.vps_li;
  if vps_li.is_empty() {
    throw!("no vps");
  }
  let vps = &vps_li[rand::random::<usize>() % vps_li.len()];
  let mut conn = conn(vps.ip, &arg.conf).await?;

  if let Some(r) =
    "SELECT id,dir,sh,preok,minute,timeout FROM i18n.cron WHERE preok<(UNIX_TIMESTAMP()/60-minute-timeout-5) LIMIT 1"
      .first::<(u64, String, String, i64, i64, i64), _>(&mut conn)
      .await?
  {
    // mariadb i18n -e "SELECT * FROM cronErr LIMIT 1"
    throw!("{}/{} not run", r.1, r.2);
  }

  OK
}
