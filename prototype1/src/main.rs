use crate_resource::{Crate, CrateKind};

mod function {
    #[derive(Debug)]
    pub struct MainFunction;

    impl MainFunction {
        pub fn new() -> Self {
            Self
        }
    }
}
mod module {
    use crate::function::MainFunction;

    pub struct Module {
        name: String,
    }

    #[derive(Debug)]
    pub struct RootModule {
        name: &'static str,
        main: Option<MainFunction>,
    }

    impl RootModule {
        pub fn new(has_main: bool) -> Self {
            Self {
                name: "crate",
                main: if has_main {
                    Some(MainFunction::new())
                } else {
                    None
                },
            }
        }
    }
}
mod crate_resource {
    use crate::module::RootModule;

    #[derive(Debug, PartialEq)]
    pub enum CrateKind {
        Executable,
        Lib,
    }

    #[derive(Debug)]
    pub struct Crate {
        name: String,
        root_module: RootModule,
        kind: CrateKind,
    }

    impl Crate {
        pub fn new(name: String, kind: CrateKind) -> Self {
            Self {
                name,
                root_module: RootModule::new(kind == CrateKind::Executable),
                kind,
            }
        }
    }
}

fn main() {
    let c = Crate::new(String::from("prototype 1"), CrateKind::Executable);

    println!("Hello, world! Generating the crate {c:?}");
}
