extern crate shell_words;
extern crate nix;

use std::process::Command;
use std::env;
use clap::{App, load_yaml};
use users::{get_current_uid};

fn run_process(process: &str, proc_args: &Vec<String>) {
    let mut command = Command::new(process)
        .args(proc_args)
        .spawn()
        .expect("Failed to spawn process");
    command.wait();
    println!("{}", get_current_uid());
    println!("{}\n", command.id());
}

fn file_process(process: &str) {
    let mut command = Command::new("sh")
        .arg("-c")
        .arg(process)
        .spawn()
        .expect("Failed to spawn process");
    command.wait();
    println!("{}", get_current_uid());
    println!("{}\n", command.id());
}

fn main() {
    // using yaml load from Clap
    let yaml = load_yaml!("first_test.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let process = &matches.value_of("process").unwrap();
    let proc_args : &Vec<String>= &matches.values_of_lossy("commands").unwrap();

    println!("Using input process: {}", process);
    println!("Using commands string: {:?}", proc_args);

    run_process(process, proc_args);

    println!("Attempting to make file");
    file_process("touch /tmp/temp.txt");

    println!("Attempting to modify file");
    file_process("echo Testing Modification >> /tmp/temp.txt");

    println!("Attempting to delete file");
    file_process("rm /tmp/temp.txt");

}
