#[macro_export]
macro_rules! watch {
  ($($name:ident),*) => {

    use alive_watch::Conf;
    use $crate::{Run};
    use aok::Result;
    use paste::paste;
    use skiplist::ordered_skiplist::OrderedSkipList;
    use alive_api::{TaskMeta,Task};

    pub struct Watch;

    paste! {
      #[derive(Debug)]
      pub enum EnumTask {
        $(
          [< $name:camel >](Task<$name::Arg>)
        ),*
      }
    }


    impl std::fmt::Display for EnumTask {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        paste! {
          match self {
            $(
              EnumTask::[< $name:camel >](_) => {
                let name = const_str::replace!(stringify!($name),"_"," ");
                write!(f, "{}", name)
              }
            )*
          }
        }
      }
    }

    impl EnumTask {
      pub fn meta(&self) -> &TaskMeta {
        paste! {
        match self {
          $(
            EnumTask::[< $name:camel >](me) => {
              &me.meta
            }
          )*
        }
        }
      }
    }

    impl Watch {
      paste! {

        pub async fn run(task: &EnumTask) -> Result<()> {
          Ok(match task {
            $(
              EnumTask::[< $name:camel >](task) => {
                $name::run(
                  &task.arg
                ).await?
              }
            )*
          })
        }

        pub async fn load(conf: &Conf) -> Result<OrderedSkipList<Run>> {
          let mut task_li = OrderedSkipList::new();
          $(
            task_li.extend($name::load(conf).await?.into_iter().map(
              |t| Run::new(
                EnumTask::[< $name:camel >](t)
              )
            ));
          )*
          Ok(task_li)
        }
      }
    }

  };
}
