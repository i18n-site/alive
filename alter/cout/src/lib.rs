use alive_alter::{Recover, Warn};
use aok::{Result, OK};

#[derive(Debug, Default)]
pub struct Alter {}

impl alive_alter::Alter for Alter {
  async fn warn(&self, warn: &Warn) -> Result<()> {
    println!("{}\n", warn);
    OK
  }

  async fn recover(&self, recover: &Recover) -> Result<()> {
    println!("{}\n", recover);
    OK
  }
}
