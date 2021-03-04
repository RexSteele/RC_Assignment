use std::process::Command;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::{App, load_yaml};
use chrono::NaiveDateTime;
use users::{get_user_by_uid, get_current_uid};

//Run given custom process, including any optional arguments
fn custom_process(process: &str, proc_args: &Vec<String>) {
    let mut command = Command::new(process)
        .args(proc_args)
        .spawn()
        .expect("Failed to spawn process");
    command.wait().expect("failed to wait on child");

    let user = get_user_by_uid(get_current_uid()).unwrap();

    println!("{}", user.name().to_string_lossy());
    println!("{}", get_current_uid());
    println!("{}\n", command.id());
}

//Run given process in bash
fn run_process(process: &str) -> u32 {
    let mut command = Command::new("sh")
        .arg("-c")
        .arg(process)
        .spawn()
        .expect("Failed to spawn process: {}");
    command.wait().expect("Failed to return from child");

    let user = get_user_by_uid(get_current_uid()).unwrap();

    println!("{}", user.name().to_string_lossy());
    println!("{}", get_current_uid());
    println!("{}\n", command.id());
    command.id()
}

//Trigger netcat udp packet using ip_address, dst_port, src_port
fn network_activity(ip_address: &str, dst_port: &str, src_port: &str) {
    let net_command = format!("echo -n \"EDR Test Packet\" | nc -p {} -u -w0 {} {}", src_port, ip_address, dst_port);
    run_process(&net_command);
}

//Trigger file activity based on pathing/file given and test_type passed
fn file_activity(test_file: &str, test_type: &str, record_file: &str) {
    let file_command : String;
    let proc_name : String;
    if test_type == "create" {
        file_command = format!("touch {}", test_file);
        proc_name = "touch".to_string();
    } else if test_type == "modify" {
        file_command = format!("echo Testing Modification >> {}", test_file);
        proc_name = "echo".to_string();
    } else {
        file_command = format!("rm {}", test_file);
        proc_name = "rm".to_string();
    }
    let proc_id = run_process(&file_command);
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let time_stamp = if test_type == "create" || test_type == "modify" {get_timestamp(get_metadata_timestamp(test_file))} else {get_timestamp(SystemTime::now())};
    let record = format!("{:?}, {}, {}, {}, {}, {}, NA, NA, NA\n", time_stamp, user.name().to_string_lossy(), proc_name, file_command, proc_id, test_type);
    write_record(record_file, & record);
}

//Write to record for telemetry  data
fn write_record(record_file: &str, data: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(record_file)
        .unwrap();
    let byte_data = data.as_bytes();
    file.write_all(byte_data).expect("Unable to write data");

    Ok(())
}

//Create record for telemetry  data
fn create_record(record_file: &str) -> std::io::Result<()> {
    File::create(record_file)?;
    let headers = "Timestamp, Username, Process Name, Process Command, Process ID, Descriptor, Dst Address:Port, Src Address:Port\n";
    write_record(record_file, headers);
    Ok(())
}

//Retrieve timestamp of last modified time of file
fn get_metadata_timestamp(test_file: &str) -> SystemTime {
    let file = File::open(test_file).unwrap();
    let f = File::metadata(&file);
    let file_time = f.unwrap().modified().unwrap();
    file_time
    // let since_epoch = file_time.duration_since(UNIX_EPOCH).unwrap();
    // let chrono_duration = ::chrono::Duration::from_std(since_epoch).unwrap();
    // let unix = NaiveDateTime::from_timestamp(0, 0);
    // let naive = unix + chrono_duration;
    // naive
}

// Convert System Time to UTC timestamp
fn get_timestamp(input_time: SystemTime) -> NaiveDateTime {
    let since_epoch = input_time.duration_since(UNIX_EPOCH).unwrap();
    let chrono_duration = ::chrono::Duration::from_std(since_epoch).unwrap();
    let unix = NaiveDateTime::from_timestamp(0, 0);
    let naive = unix + chrono_duration;
    naive
}

fn main() {
    // using yaml load from Clap
    let yaml = load_yaml!("edr_test.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let record_file = &matches.value_of("record_file").unwrap();
    let ip_address = &matches.value_of("ip_address").unwrap();
    let dst_port = &matches.value_of("dst_port").unwrap();
    let src_port = &matches.value_of("src_port").unwrap();
    let test_file = &matches.value_of("test_file").unwrap();
    let process = &matches.value_of("process").unwrap();
    let proc_args : &Vec<String>= &matches.values_of_lossy("commands").unwrap();

    println!("Using record file: {}", record_file);
    println!("\nUsing destination ip address: {}", ip_address);
    println!("Using destination port: {}", dst_port);
    println!("Using source port: {}", src_port);
    println!("Using file: {}", test_file);
    println!("Using input process: {}", process);
    println!("Using commands string: {:?}", proc_args);

    println!("Creating record file");
    create_record(record_file);

    println!("Attempting network activity");
    network_activity(ip_address, dst_port, src_port);

    println!("Attempting to make file");
    file_activity(test_file, "create", record_file);

    println!("Attempting to modify file");
    file_activity(test_file, "modify", record_file);
    get_metadata_timestamp(test_file);

    println!("Attempting to delete file");
    file_activity(test_file, "remove", record_file);

    custom_process(process, proc_args);

}
