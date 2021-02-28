#[macro_use]
extern crate clap;
use clap::App;

fn main() {

    let yaml = load_yaml!("clap_test.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let commands = matches.value_of("commands").unwrap_or("default.conf");
    println!("Value for commands: {}", commands);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("process").unwrap());

}
