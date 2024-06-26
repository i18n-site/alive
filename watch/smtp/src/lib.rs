use std::{collections::HashMap, net::IpAddr};

use alive_api::Task;
use aok::{ensure, Result, OK};
use mail_send::SmtpClientBuilder;

#[derive(Clone, serde::Deserialize, Debug)]
pub struct SmtpConf {
  host: String,
  user: String,
  password: String,
  port: u16,
  implicit_tls: bool,
}

#[derive(Debug)]
pub struct Arg {
  pub ip: IpAddr,
  pub conf: SmtpConf,
}

const DURATION: u64 = 60;

pub async fn load(wc: &alive_watch::Conf) -> Result<Vec<Task<Arg>>> {
  let c: HashMap<String, SmtpConf> = alive_watch::yml!(wc);
  let mut r = Vec::new();
  for (cluster, smtp_conf) in c {
    for (vps, ip) in wc.cluster(cluster)?.iter() {
      macro_rules! push {
        ($attr:ident, $v:ident, $tag:ident) => {
          for i in &ip.$attr {
            r.push(Task::new(
              vps.clone(),
              Arg {
                ip: IpAddr::$v(*i),
                conf: smtp_conf.clone(),
              },
              [stringify!($tag).into()],
              DURATION,
            ))
          }
        };
      }
      push!(ipv4_li, V4, IPV4);
      push!(ipv6_li, V6, IPV6);
    }
  }
  Ok(r)
}

pub async fn run(arg: &Arg) -> Result<()> {
  let ip = arg.ip;
  let conf = &arg.conf;
  let port = conf.port;
  let host = conf.host.as_str();
  let smtp = SmtpClientBuilder::new_bind_ip(host, ip, port)
    .implicit_tls(conf.implicit_tls)
    .credentials(mail_send::Credentials::<&str>::Plain {
      username: conf.user.as_str(),
      secret: conf.password.as_str(),
    });
  let ehlo = smtp.connect().await?.ehlo(host).await?;
  let hostname = ehlo.hostname;
  ensure!(
    *host == hostname,
    format!("smtp {ip}:{port} hostname {hostname} != {host}",)
  );
  OK
}
