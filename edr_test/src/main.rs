use std::process::Command;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::net::IpAddr;

//External crates
use clap::{App, load_yaml};
use chrono::NaiveDateTime;
use users::{get_user_by_uid, get_current_uid};
use local_ipaddress;

//Run given process in bash
fn run_process(process: &str) -> u32 {
    let mut command = Command::new("sh")
        .arg("-c")
        .arg(process)
        .spawn()
        .expect("Failed to spawn process: {}");
    command.wait().expect("Failed to return from child");

    command.id()
}

//Trigger netcat TCP packet using ip_address, dst_port, src_port
fn network_activity(dst_address: &str, dst_port: &str, src_port: &str, record_file: &str) {
    let src_ip = if check_dst(dst_address) {local_ipaddress::get().unwrap()} else {dst_address.to_string()};
    let data = "\"EDR Test Packet\"";
    let net_command = format!("echo {} | nc -p {} {} {}", data, src_port, dst_address, dst_port);

    let time_stamp = get_timestamp(SystemTime::now());
    let proc_id = run_process(&net_command);

    let user = get_user_by_uid(get_current_uid()).unwrap();

    let record = format!("{:?}, {}, {}, {}, {}, {}, {}:{}, {}:{}, {}, {}\n", time_stamp, user.name().to_string_lossy(),
                        "nc", net_command, proc_id, "network", src_ip, src_port, dst_address, dst_port, "TCP", data);
    write_record(record_file, &record).expect("Unable to write to log file");
}

//Check if dst address is loopback or not
fn check_dst(dst_address: &str) -> bool {
    let dst_ip : IpAddr = dst_address.parse().expect("Unable to parse dst IP addr");
    let mut status : bool = true;
    if dst_ip.is_loopback() {
        status = false;
    }
    status
}

//Trigger file activity based on pathing/file given and test_type passed
fn prep_process(process_arg: &str, test_type: &str, record_file: &str) {
    let process_command : String;
    let proc_name : String;

    if test_type == "create" {
        process_command = format!("touch {}", process_arg);
        proc_name = "touch".to_string();
    } else if test_type == "modify" {
        process_command = format!("chmod +x {}", process_arg);
        proc_name = "chmod".to_string();
    } else if test_type == "remove"{
        process_command = format!("rm {}", process_arg);
        proc_name = "rm".to_string();
    } else {
        process_command = process_arg.to_string();
        proc_name = process_command.split_whitespace().next().unwrap().to_string();
    }

    let mut time_stamp = get_timestamp(SystemTime::now());
    let proc_id = run_process(&process_command);
    if test_type == "create" || test_type == "modify" {time_stamp = get_timestamp(get_metadata_timestamp(process_arg))};

    let user = get_user_by_uid(get_current_uid()).unwrap();

    let record = format!("{:?}, {}, {}, {}, {}, {}, NA, NA, NA, NA\n", time_stamp, user.name().to_string_lossy(), proc_name, process_command, proc_id, test_type);
    write_record(record_file, &record).expect("Unable to write to log file");
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

//Create record for telemetry  data, overwrites if already exists
fn create_record(record_file: &str) -> std::io::Result<()> {
    File::create(record_file)?;
    let headers = "Timestamp, Username, Process Name, Process Command, Process ID, Descriptor, Src Address:Port, Dst Address:Port, Protocol, Network Data\n";
    write_record(record_file, headers).expect("Unable to write to log file");
    Ok(())
}

//Retrieve timestamp of last modified time of file
fn get_metadata_timestamp(test_file: &str) -> SystemTime {
    let file = File::open(test_file).unwrap();
    let f = File::metadata(&file);
    let file_time = f.unwrap().modified().unwrap();
    file_time
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
    let record_file = &matches.value_of("csv_file").unwrap();
    let ip_address = &matches.value_of("ip_address").unwrap();
    let dst_port = &matches.value_of("dst_port").unwrap();
    let src_port = &matches.value_of("src_port").unwrap();
    let test_file = &matches.value_of("test_file").unwrap();
    let process = &matches.value_of("process").unwrap();
    let proc_args : &Vec<String>= &matches.values_of_lossy("commands").unwrap_or([].to_vec());

    create_record(record_file).expect("Unable to create log file");

    network_activity(ip_address, dst_port, src_port, record_file);

    prep_process(test_file, "create", record_file);

    prep_process(test_file, "modify", record_file);

    prep_process(test_file, "remove", record_file);

    let proc_args_str = proc_args.join(" ");
    let full_process = if !proc_args_str.is_empty() {format!("{} {}", process, proc_args_str)} else {process.to_string()};
    prep_process(&full_process, "custom", record_file);

}
