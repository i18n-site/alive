use std::net::IpAddr;

use alive_api::Task;
use aok::{ensure, Result, OK};

#[derive(serde::Deserialize, Debug)]
pub struct Conf {
  pub cluster: Vec<String>,
}

#[derive(Debug)]
pub struct Arg {
  pub ip: IpAddr,
}

// genv::s!(
//   IPV6_PROXY_USER,
//   IPV6_PROXY_PASSWD,
//   IPV6_PROXY_PORT,
//   IPV6_PROXY_TEST_URL,
//   IPV6_PROXY_TEST_RESULT
// );

const DURATION: u64 = 60;

pub async fn load(wc: &alive_watch::Conf) -> Result<Vec<Task<Arg>>> {
  let c: Conf = alive_watch::yml!(wc);
  let mut r = Vec::new();
  for i in c.cluster {
    for (host, ip) in wc.cluster(i)?.iter() {
      macro_rules! push {
        ($attr:ident, $v:ident, $tag:ident) => {
          for i in &ip.$attr {
            r.push(Task::new(
              host.clone(),
              Arg { ip: IpAddr::$v(*i) },
              [stringify!($tag).into()],
              DURATION,
            ))
          }
        };
      }
      push!(ipv4_li, V4, IPV4);
    }
  }
  Ok(r)
}

pub async fn run(arg: &Arg) -> Result<()> {
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
  OK
}
