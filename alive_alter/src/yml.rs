use std::path::Path;

use aok::Result;

#[macro_export]
macro_rules! yml {
  ($dir:expr) => {
    $crate::yml(
      $dir,
      $crate::const_str::concat!(env!("CARGO_PKG_NAME"), ".yml"),
    )?
  };
}

pub fn yml<T: serde::de::DeserializeOwned>(dir: &Path, file: &str) -> Result<T> {
  let fp = dir.join(file);
  let yml = ifs::r(fp)?;
  if yml.is_empty() {
    tracing::warn!("{file} IS EMPTY");
  }
  Ok(serde_yaml::from_slice(&yml)?)
}
