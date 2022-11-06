use std::{
    env, fs,
    io::{stdin, Write},
    path::PathBuf,
    process::{exit, Command},
};
use tempfile::Builder as TempfileBuilder;

const TEMPLATE: &str = include_str!("template.rs");

fn main() {
    let args = &env::args().collect::<Vec<String>>()[1..].to_vec();
    let script_file = if args.len() > 0 {
        PathBuf::from(&args[0])
    } else {
        println!("Enter the path to the script: ");
        let mut line = String::new();

        stdin().read_line(&mut line).expect("failed to get input.");

        PathBuf::from(line)
    };

    if !script_file.exists() || !script_file.is_file() {
        println!("file not found.");
        exit(1)
    }

    let script_file_contents = fs::read_to_string(&script_file).unwrap();

    let script_file_lines = script_file_contents.split('\n').collect::<Vec<&str>>();

    let executable = if script_file_lines[0].starts_with("rem S2B:")
        || script_file_lines[0].starts_with("#S2B:")
        || script_file_lines[0].starts_with("//S2B:")
    {
        let first_line = script_file_lines.first().unwrap().to_string();

        first_line
            .replacen("rem S2B:", "", 1)
            .replacen("#S2B:", "", 1)
            .replacen("//S2B:", "", 1)
    } else if script_file_lines.len() > 1 && script_file_lines[1].starts_with("rem S2B:") {
        script_file_lines[1].replacen("rem S2B:", "", 1)
    } else if script_file_lines[0].starts_with("#!") {
        script_file_lines[0].replacen("#!", "", 1)
    } else {
        println!("no shebang/S2B annotation found");
        exit(1)
    };

    let mut tmp_rs_file = TempfileBuilder::new()
        .prefix("shell2bin-temp")
        .suffix(".rs")
        .rand_bytes(5)
        .tempfile()
        .expect("failed to create temp file");

    tmp_rs_file
        .write(
            TEMPLATE
                .replace(
                    "${PATH}",
                    env::current_dir()
                        .unwrap()
                        .join(&script_file)
                        .to_str()
                        .unwrap()
                        .trim(),
                )
                .replace("${EXE}", &executable.trim())
                .as_bytes(),
        )
        .expect("failed to write to temp file.");

    Command::new("rustc")
        .arg(tmp_rs_file.path().to_str().unwrap())
        .arg(format!(
            "-o{}",
            if args.len() > 1 {
                &args[1]
            } else {
                script_file.file_stem().unwrap().to_str().unwrap()
            }
        ))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    println!("done.");
}
