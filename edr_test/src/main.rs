extern crate shell_words;

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
    let yaml = load_yaml!("edr_test.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let ip_address = &matches.value_of("ip_address").unwrap();
    let dst_port = &matches.value_of("dst_port").unwrap();
    let src_port = &matches.value_of("src_port").unwrap();
    let file_directory = &matches.value_of("file_directory").unwrap();
    let process = &matches.value_of("process").unwrap();
    let proc_args : &Vec<String>= &matches.values_of_lossy("commands").unwrap();

    println!("Using destination ip address: {}", ip_address);
    println!("Using destination port: {}", dst_port);
    println!("Using source port: {}", src_port);
    println!("Using file directory: {}", file_directory);
    println!("Using input process: {}", process);
    println!("Using commands string: {:?}", proc_args);

    run_process(process, proc_args);

    println!("Attempting to make file");
    file_process("touch /tmp/temp.txt");

    println!("Attempting to modify file");
    file_process("echo Testing Modification >> /tmp/temp.txt");

    println!("Attempting to delete file");
    file_process("rm /tmp/temp.txt");

    println!("Attempting network activity");
    file_process("echo -n \"hello\" | nc -p 700 -4u -w0 localhost 700");

}
