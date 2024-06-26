[‼️]: ✏️README.mdt

# alive_watch

```rust
use std::path::PathBuf;

use alive_watch::Conf;
use aok::{Result, OK};
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[test]
fn test() -> Result<()> {
  let dir: PathBuf = std::env!("CARGO_MANIFEST_DIR").into();
  let conf = Conf::new(dir.parent().unwrap().join("conf"));
  info!("dir: {:?}", conf);
  for i in conf.cluster("ol")?.iter() {
    dbg!(i);
  }
  OK
}
```
