#[cfg(feature = "yml")]
pub use const_str;

#[cfg(feature = "yml")]
mod yml;

#[cfg(feature = "yml")]
pub use yml::yml;

#[cfg(feature = "denoise")]
pub mod denoise;

#[cfg(feature = "title_txt")]
mod title_txt;

#[cfg(feature = "title_txt")]
pub use title_txt::title_txt;

mod warn;
pub use warn::Warn;

mod recover;
pub use recover::Recover;

pub enum Msg {
  Warn(Warn),
  Recover(Recover),
  None,
}

#[macro_export]
macro_rules! display {
  ($self:ident,$f:ident) => {{
    write!($f, " {} : {}", $self.watch_name, $self.host)?;
    if !$self.tag_li.is_empty() {
      write!($f, " ")?;
      for (n, i) in $self.tag_li.iter().enumerate() {
        if n > 0 {
          write!($f, " ")?;
        }
        write!($f, "{}", i)?;
      }
    }
    write!($f, " ⏲ {}", sts::readable($self.duration))?;
  }};
}

use std::path::Path;

use aok::Result;

pub trait Load: Sized {
  fn load(dir: &Path) -> impl std::future::Future<Output = Result<Self>> + Send;
}

impl<T: Default> Load for T {
  async fn load(_dir: &Path) -> Result<Self> {
    Ok(Default::default())
  }
}

pub trait Alter {
  fn warn(&self, warn: &Warn) -> impl std::future::Future<Output = Result<()>> + Send;

  fn recover(&self, recover: &Recover) -> impl std::future::Future<Output = Result<()>> + Send;
}
