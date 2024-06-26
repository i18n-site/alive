[‼️]: ✏️watch/ipv6_proxy/README.mdt

# ipv6_proxy

```rust
use aok::{Result, OK};
use static_init::constructor;
use tracing::info;

#[constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::test]
async fn test() -> Result<()> {
  info!("{}", 123456);
  OK
}
```
