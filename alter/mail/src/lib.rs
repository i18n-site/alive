use std::path::Path;

use alive_alter::{denoise, title_txt, Recover, Warn};
use aok::{Result, OK};

#[derive(serde::Deserialize, Debug)]
pub struct Conf {
  pub name: String,
  pub to: Vec<String>,
}

#[derive(Debug)]
pub struct Alter {
  conf: Conf,
}

impl Alter {
  pub async fn load(dir: &Path) -> Result<Self> {
    let conf: Conf = alive_alter::yml!(dir);
    Ok(Self { conf })
  }

  pub fn _send(&self, title: &str, txt: &str) {
    let conf = &self.conf;
    for to in conf.to.iter() {
      xsmtp::send_bg(&conf.name, to.to_owned(), title, txt, "");
    }
  }

  pub async fn warn(&self, warn: &Warn) -> Result<()> {
    if denoise::should_send(warn.times, warn.first_warn) {
      let warn = warn.to_string();
      let (title, txt) = title_txt(&warn);
      self._send(title, txt);
    }
    OK
  }

  pub async fn recover(&self, recover: &Recover) -> Result<()> {
    if recover.err >= recover.first_warn {
      self._send(&recover.to_string(), "");
    }
    OK
  }
}
