use std::{fs::File, io::Write, process::Command};

struct MainFunction {
    source_code: String,
}

const MAIN_FILE_NAME: &str = "tmp_main";
const RUST_EXTENSION: &str = ".rs";
const EXE_EXTENSION: &str = ".exe";
impl MainFunction {
    pub fn new() -> Self {
        Self {
            source_code: r#"
fn main(){
    println!("Wohoo, executing from Software CAD! Hello world!");
}
"#
            .to_string(),
        }
    }

    fn pre_compile(&self) -> &str {
        self.source_code.as_str()
    }

    fn to_tmp_file(&self) -> Result<(), &'static str> {
        let pre_compiled_source_code = self.pre_compile().to_string();
        let mut file = File::create(format!("{}{}", MAIN_FILE_NAME, RUST_EXTENSION)).unwrap();
        file.write(pre_compiled_source_code.as_bytes()).unwrap();

        Ok(())
    }

    pub fn run(&self) {
        let _ = self.to_tmp_file();
        println!("Compiling...");
        Command::new("rustc")
            .arg(format!("{}{}", MAIN_FILE_NAME, RUST_EXTENSION))
            .spawn()
            .expect("Run failed!");
        println!("Executing the program...");
        Command::new(format!("./{}{}", MAIN_FILE_NAME, EXE_EXTENSION))
            .spawn()
            .expect("Compiled program run failed!");
    }
}

fn main() {
    let my_code = MainFunction::new();
    my_code.run();
}

// todo: implement a very basic version control for the main hello world program.
