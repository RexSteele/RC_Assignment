extern crate shell_words;
use std::process::Command;

// use clap::{App, load_yaml};

// fn run_process(process: &str) {
//     let output = Command::new(process)
//         .spawn()
//         .expect("ls command failed to start\n");
//
//     let hello = output.stdout;
//     println!("{:#?}", hello);
// }

fn main() {
    // let yaml = load_yaml!("first_test.yml");
    // let matches = App::from_yaml(yaml).get_matches();
    //
    // // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let commands = &matches.value_of("commands").unwrap_or("default.conf");
    // let parsedCommands= &shell_words::join(commands.chars());
    // println!(parsedCommands);
    //
    // // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // // required we could have used an 'if let' to conditionally get the value)
    // println!("Using input file: {}", matches.value_of("process").unwrap());

    let argv = &["ls", "-l"];

    println!("Executing: {}", shell_words::join(argv));

    std::process::Command::new(&argv[0])
        .args(&argv[1..])
        .spawn()
        .expect("failed to start subprocess")
        .wait()
        .expect("failed to wait for subprocess");
}
