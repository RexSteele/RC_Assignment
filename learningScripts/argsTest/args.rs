use std::env;
use std::process::Command;

fn run_process(process: &str, arr: &[&str]) {
    let output = Command::new("process")
        .arg(process)
        .args(arr)
        .spawn()
        .expect("ls command failed to start\n");

    let hello = output.stdout;
    println!("{:#?}", hello);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    let commands = &args[2..];
    run_process(&args[1], &commands);
}
