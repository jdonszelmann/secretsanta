use std::process::{Command, Stdio};
use std::io::{Write, Read};
use std::env::current_exe;
use std::fs::File;
use std::path::PathBuf;

const SPACE: char = '\u{200b}';
static mut MANUAL_ID: usize = 0;
const MANUAL_MAIN_FILE: &'static str = "main";
const MANUAL_DIR: &'static str = ".manual";

fn get_manual_dir() -> PathBuf{
    let executable_file = current_exe()
        .expect("Couln't find executable directory");

    let executable_dir = executable_file.parent()
        .expect("Couldn't access parent directory of executable");

    let manual_dir = executable_dir.join(MANUAL_DIR);
    std::fs::create_dir_all(&manual_dir);

    manual_dir
}

pub fn get_manual_id() {
    let manual_dir = get_manual_dir();

    let mainfile_path = manual_dir.join(MANUAL_MAIN_FILE);
    let manualid= match File::open(&mainfile_path) {
        Ok(mut mainfile) => {
            let mut buf = String::new();
            mainfile.read_to_string(&mut buf);

            buf.chars()
                .take(10)
                .filter(|c| c == &SPACE)
                .count()
        }
        Err(_) => 0,
    };


    unsafe{ MANUAL_ID = manualid };
}

pub fn find_editor() -> String {
    std::env::var("EDITOR").unwrap_or("vim".into())
}

pub fn run_manual() {
    let editor = find_editor();

    println!("Starting manual");

    let manual_dir = get_manual_dir();


    generate_manual(&manual_dir);


    let mainfile_path = manual_dir.join(MANUAL_MAIN_FILE);
    let child = Command::new(&editor)
        .arg(&mainfile_path)
        .status()
        .expect(&format!("Couldn't start editor: {}", &editor));
}

fn generate_manual(dir: &PathBuf) {
    let manual_dir = get_manual_dir();

    let mainfile_path = manual_dir.join(MANUAL_MAIN_FILE);
    let mut mainfile = File::create(&mainfile_path)
        .expect("Couldn't create manual file");

    match unsafe{ MANUAL_ID } {

        _ => ()
    }
}