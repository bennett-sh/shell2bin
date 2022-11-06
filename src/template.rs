use std::{env::temp_dir , process::Command, fs, path::PathBuf};

fn main() {
    let file_contents = include_str!(r"${PATH}");
    let orig_path = PathBuf::from(r"${PATH}");
    let name = orig_path.file_name().unwrap();
    let path = temp_dir().join(name);
    let exe_cmd = r"${EXE}".split(' ').collect::<Vec<&str>>();
    let exe = &exe_cmd.first();
    let exe_args = &exe_cmd[1..].to_vec();

    fs::write(&path, &file_contents).unwrap();

    Command::new(exe.unwrap())
        .current_dir(&orig_path.parent().unwrap())
        .args(exe_args)
        .arg(&path)
        .spawn()
        .expect("failed to run script (from shell2bin).")
        .wait()
        .unwrap();

    fs::remove_file(&path).unwrap();
}
