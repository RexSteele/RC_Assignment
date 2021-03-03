extern crate shell_words;

use std::process::Command;
use std::env;
use clap::{App, load_yaml};
use users::{get_user_by_uid, get_current_uid};

fn custom_process(process: &str, proc_args: &Vec<String>) {
    let mut command = Command::new(process)
        .args(proc_args)
        .spawn()
        .expect("Failed to spawn process");
    let ecode = command.wait().expect("failed to wait on child");
    println!("{}", ecode);
    let user = get_user_by_uid(get_current_uid()).unwrap();
    println!("{}", user.name().to_string_lossy());
    println!("{}", get_current_uid());
    println!("{}\n", command.id());
}

fn run_process(process: &str) {
    let mut command = Command::new("sh")
        .arg("-c")
        .arg(process)
        .spawn()
        .expect("Failed to spawn process");
    let ecode = command.wait().expect("failed to wait on child");
    println!("{}", ecode);
    let user = get_user_by_uid(get_current_uid()).unwrap();
    println!("{}", user.name().to_string_lossy());
    println!("{}", get_current_uid());
    println!("{}\n", command.id());
}

fn network_activity(ip_address: &str, dst_port: &str, src_port: &str) {
    let net_command = format!("echo -n \"EDR Test Packet\" | nc -p {} -u -w0 {} {}", src_port, ip_address, dst_port);
    run_process(&net_command);
}

fn file_activity(test_file: &str, test_type: &str) {
    let file_command : String;
    if test_type == "create" {
        file_command = format!("touch {}", test_file);
    } else if test_type == "modify" {
        file_command = format!("echo Testing Modification >> {}", test_file);
    } else {
        file_command = format!("rm {}", test_file);
    }
    run_process(&file_command);
}

fn main() {
    // using yaml load from Clap
    let yaml = load_yaml!("edr_test.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let ip_address = &matches.value_of("ip_address").unwrap();
    let dst_port = &matches.value_of("dst_port").unwrap();
    let src_port = &matches.value_of("src_port").unwrap();
    let test_file = &matches.value_of("test_file").unwrap();
    let process = &matches.value_of("process").unwrap();
    let proc_args : &Vec<String>= &matches.values_of_lossy("commands").unwrap();

    println!("Using destination ip address: {}", ip_address);
    println!("Using destination port: {}", dst_port);
    println!("Using source port: {}", src_port);
    println!("Using file: {}", test_file);
    println!("Using input process: {}", process);
    println!("Using commands string: {:?}", proc_args);

    println!("Attempting network activity");
    network_activity(ip_address, dst_port, src_port);

    println!("Attempting to make file");
    file_activity(test_file, "create");

    println!("Attempting to modify file");
    file_activity(test_file, "modify");

    println!("Attempting to delete file");
    file_activity(test_file, "remove");

    custom_process(process, proc_args);

}
