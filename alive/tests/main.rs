// use alive_plugin::{alter::Alter, watch::Watch};
// use aok::{Result, OK};
//
// #[tokio::test]
// async fn test() -> Result<()> {
//   let dir = std::env::current_dir()?;
//   let conf = dir.parent().unwrap().join("conf");
//   let mut alive = alive::Alive::<Alter, Watch>::load(&conf).await?;
//   // let api = alive.api.clone();
//
//   // tokio::spawn(async move {
//   //   loop {
//   //     let li = api.proto();
//   //     dbg!(li);
//   //     sleep(Duration::from_secs(60)).await;
//   //   }
//   // });
//
//   alive.run().await?;
//   // let mut alive = Alive::new();
//   // alive.task.insert(Task {
//   //   ts: 1,
//   //   kind: Kind::CnameFlatten,
//   //   duration: 40,
//   // });
//   // alive.task.insert(Task {
//   //   ts: 3,
//   //   kind: Kind::CnameFlatten,
//   //   duration: 40,
//   // });
//   // alive.task.insert(Task {
//   //   ts: 2,
//   //   kind: Kind::CnameFlatten,
//   //   duration: 40,
//   // });
//   // dbg!(&alive.task);
//   OK
// }
