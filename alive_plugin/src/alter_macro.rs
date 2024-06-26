#[macro_export]
macro_rules! alter {
  ($($name:ident),*) => {
    use aok::{OK,Result};
    use std::path::Path;
    use alive_alter::Recover;
    use alive_alter::{Load,Warn};
    use alive_alter::Alter as _Alter;


    #[derive(Debug)]
    pub struct Alter {
      $(
        pub $name: $name::Alter
      ),*
    }

    impl Load for Alter {
      async fn load(dir: &Path) -> Result<Alter> {
        Ok(Alter {
          $(
            $name: $name::Alter::load(dir).await?
          ),*
        })
      }
    }


    impl _Alter for Alter {
      async fn warn(&self, warn: &Warn) -> Result<()> {
        let rli = tokio::join!(
          $(
            self.$name.warn(warn)
          ),*
        );
        $(
          ${ignore($name)}
          $crate::xerr::log!(rli.${index()});
        )*
        OK
      }

      async fn recover(&self, recover: &Recover) -> Result<()> {
        let rli = tokio::join!(
          $(
            self.$name.recover(recover)
          ),*
        );
        $(
          ${ignore($name)}
          $crate::xerr::log!(rli.${index()});
        )*
        OK
      }
    }
  };
}
