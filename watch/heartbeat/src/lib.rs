use alive_api::Task;
use aok::{throw, Result, OK};
pub use coarsetime::Clock;
use dashmap::DashMap;
use mysql_async::{prelude::Query, Conn, Opts, OptsBuilder, SslOpts};
use tokio::time::{sleep, Duration};

#[static_init::dynamic]
pub static ID_EXPIRE: DashMap<u32, u64> = DashMap::new();

#[derive(serde::Deserialize, Debug)]
pub struct Conf {
  mysql: String,
}

#[derive(Debug)]
pub struct Arg {
  pub id: u32,
}

pub const DURATION: u64 = 60;

pub async fn load(wc: &alive_watch::Conf) -> Result<Vec<Task<Arg>>> {
  let c: Conf = alive_watch::yml!(wc);
  let opts = Opts::from_url(&format!("mysql://{}", c.mysql))?;
  let opts = OptsBuilder::from_opts(opts);
  let opts = opts.ssl_opts(SslOpts::default().with_danger_accept_invalid_certs(true));
  let opts: Opts = opts.into();
  let mut conn = Conn::new(opts.clone()).await?;
  let li: Vec<(u32, String, String, u64)> =
    "SELECT h.id,vps.name,h.name,expire FROM heartbeat h,vps WHERE vps.id=h.vps_id"
      .fetch(&mut conn)
      .await?;

  tokio::spawn(async move {
    loop {
      if let Ok(mut conn) = xerr::ok!(Conn::new(opts.clone()).await) {
        if let Ok::<Vec<(u32, u64)>, _>(li) =
          xerr::ok!("SELECT id,expire FROM heartbeat".fetch(&mut conn).await)
        {
          for (id, expire) in li {
            ID_EXPIRE.insert(id, expire);
          }
        }
      }
      sleep(Duration::from_secs(DURATION)).await;
    }
  });

  Ok(
    li.into_iter()
      .map(|(id, vps, name, expire)| {
        ID_EXPIRE.insert(id, expire);
        Task::new(name, Arg { id }, [vps], DURATION + 9)
      })
      .collect(),
  )
}

pub async fn run(arg: &Arg) -> Result<()> {
  let now: u64 = Clock::now_since_epoch().as_secs();
  let expire = *ID_EXPIRE.get(&arg.id).unwrap();
  if expire < now {
    let diff = now - expire;
    throw!("expire {diff}s");
  }
  OK
}
