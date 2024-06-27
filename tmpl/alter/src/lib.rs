use std::path::Path;

use alive_alter::{Recover, Warn};
// use alive_alter::{denoise, title_txt, Recover, Warn};
use aok::{Result, OK};

// #[derive(serde::Deserialize, Debug)]
// pub struct Conf {
//   pub to: Vec<String>,
// }

#[derive(Debug, Default)]
pub struct Alter {
  // conf: Conf,
}

// impl Alter {
//   pub async fn _send(&self, title: &str, txt: &str) -> Result<()> {
//     let r = push::send(title, txt, &*ALIVE_URL).await;
//     OK
//   }
// }

impl alive_alter::Alter for Alter {
  // async fn load(dir: &Path) -> Result<Self> {
  //   // for ignore unused warn
  //   let _ = dir.as_os_str();
  // let conf: Conf = alive_alter::yml!(dir);
  //   Ok(Self {
  //     conf
  //   })
  // }

  async fn warn(&self, warn: &Warn) -> Result<()> {
    println!("{}", warn);
    // if denoise::should_send(warn.times,warn.first_warn) {
    //   let warn = warn.to_string();
    //   let (title, txt) = title_txt(&warn);
    //   self._send(&title, &txt).await?;
    // }
    OK
  }

  async fn recover(&self, recover: &Recover) -> Result<()> {
    println!("{}", recover);
    // self._send(&recover.to_string(), "").await?;
    OK
  }
}
