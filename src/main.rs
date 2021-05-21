use u_risc_interpreter::{Cpu};
use std::env;
mod run;
mod debug;
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        None => print_help(""),
        Some(command) => {match command.as_str() {
            "run" => {run::main(args)}
            "debug" => {debug::main(args)}
            _ => {print_help("")}
        }}
    }
}

fn print_help(err: &str) {
    print!("{}", err);
    println!("U-RISC-LAUNCHER v{} using U-RISC-INTERPRETER v{}\nAvailable commands:\n  run FILENAME\n  debug FILENAME", env!("CARGO_PKG_VERSION"), u_risc_interpreter::get_version());
}