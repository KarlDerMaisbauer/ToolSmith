use clap::{Arg, Command};
use std::env;
use std::path::Path;
use std::process::*;
const VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() {
    let program_name = env::args().next().unwrap();
    let program_path = Path::new(&program_name);
    println!("me: {:?}", program_name);
    let matches = Command::new("ToolSmith")
        .version(VERSION)
        .arg(
            Arg::new("File")
                .help("executable to process")
                .required(true)
                .value_hint(clap::ValueHint::FilePath),
        )
        .arg(
            Arg::new("Move")
                .short('m')
                .long("move")
                .help("moves file instead of copying it")
                .conflicts_with("Delete")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("Delete")
                .short('d')
                .long("delete")
                .help("deletes executable from usr/local/bin")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();
    let file_to_handle = matches.get_one::<String>("File").unwrap();
    let path_to_handle = Path::new(&file_to_handle);
    if path_to_handle.file_name() == program_path.file_name() {
        println!("ToolSmith cannot replace itself yet");
        exit(1);
    }

    let bin_path = Path::new("/usr/local/bin");
    if matches.get_flag("Delete") {
        let file_path = bin_path.join(file_to_handle);
        if !file_path.exists() {
            println!("{:?} does not exist", file_path);
            exit(1);
        }
        if !file_path.is_file() {
            println!("{:?} is not a file", file_path);
            exit(1);
        }
        let status = std::process::Command::new("sudo")
            .args(["rm", file_path.to_str().unwrap()])
            .status()
            .expect("Failed to delete the file");
        if status.success() {
            println!("{:?} deleted successfully.", file_to_handle);
        }
    } else {
        if !path_to_handle.exists() {
            println!("{:?} does not exist", path_to_handle);
            exit(1);
        }
        if !path_to_handle.is_file() {
            println!("{:?} is not a file", path_to_handle);
            exit(1);
        }
        let mut base_command = "cp";
        if matches.get_flag("Move") {
            base_command = "mv"
        }

        let file_name = path_to_handle.file_name().unwrap();
        let dst = bin_path.join(file_name);

        std::process::Command::new("sudo")
            .args([
                base_command,
                path_to_handle.to_str().unwrap(),
                dst.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to move/copy the file");
        println!("{:?} moved/copied successfully.", file_to_handle);

        std::process::Command::new("sudo")
            .args(["chmod", "+x", dst.to_str().unwrap()])
            .status()
            .expect("Failed to make file executable");
        println!("{:?} is now ready to use", file_to_handle);
    }
}
