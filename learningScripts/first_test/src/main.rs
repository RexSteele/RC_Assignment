extern crate shell_words;
use std::process::Command;
use std::env;

use clap::{App, load_yaml};

fn run_process(process: &str, procArgs: &Vec<String>) {
    let command = Command::new(process)
        .args(procArgs)
        .spawn()
        .expect("Failed to spawn process");
    println!("{}", command.id())
    // if let Ok(child) = command.spawn() {
    //     println!("Child's ID is {}", child.id());
    // } else {
    //     println!("process didn't start");
    // }
}

fn main() {
    // using yaml load from Clap
    let yaml = load_yaml!("first_test.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let process = &matches.value_of("process").unwrap();
    let procArgs : &Vec<String>= &matches.values_of_lossy("commands").unwrap();
    println!("{:?}", procArgs);


    println!("Using input process: {}", matches.value_of("process").unwrap());
    println!("Using commands string: {}", matches.value_of("commands").unwrap());

    run_process(process, procArgs);

}
