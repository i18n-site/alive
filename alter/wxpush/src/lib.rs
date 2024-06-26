use alive_alter::{denoise, title_txt, Recover, Warn};
use aok::{Result, OK};

genv::s!(ALIVE_URL);

#[derive(Debug, Default)]
pub struct Alter {}

impl Alter {
  pub async fn _send(&self, title: &str, txt: &str) -> Result<()> {
    push::send(title, txt, &*ALIVE_URL).await
  }
}

impl alive_alter::Alter for Alter {
  async fn warn(&self, warn: &Warn) -> Result<()> {
    if denoise::should_send(warn.times, warn.first_warn) {
      let warn = warn.to_string();
      let (title, txt) = title_txt(&warn);
      let title = format!("故障 {title}");
      self._send(&title, txt).await?;
    }
    OK
  }

  async fn recover(&self, recover: &Recover) -> Result<()> {
    if recover.err >= recover.first_warn {
      let txt = format!("恢复 {}", recover);
      self._send(&txt, &txt).await?;
    }
    OK
  }
}
