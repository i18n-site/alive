#[macro_export]
macro_rules! yml {
  ($conf:ident) => {
    $conf.yml($crate::const_str::concat!(env!("CARGO_PKG_NAME"), ".yml"))?
  };
}
