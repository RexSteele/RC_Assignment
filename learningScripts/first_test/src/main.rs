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

fn make_file() {
    let mut command = Command::new("sh")
        .arg("-c")
        .arg("touch /tmp/temp.txt")
        .spawn()
        .expect("Failed to spawn process");
    command.wait();
    println!("{}", get_current_uid());
    println!("{}\n", command.id());
}

fn modify_file() {
    let mut command = Command::new("sh")
        .arg("-c")
        .arg("echo Testing Modification >> /tmp/temp.txt")
        .spawn()
        .expect("Failed to spawn process");
    command.wait();
    println!("{}", get_current_uid());
    println!("{}\n", command.id());
}

fn delete_file() {
    let mut command = Command::new("sh")
        .arg("-c")
        .arg("rm /tmp/temp.txt")
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
    println!("{:?}", proc_args);


    println!("Using input process: {}", matches.value_of("process").unwrap());
    println!("Using commands string: {}", matches.value_of("commands").unwrap());

    run_process(process, proc_args);

    println!("Attempting to make file");
    make_file();

    println!("Attempting to modify file");
    modify_file();

    println!("Attempting to delete file");
    delete_file();

}
