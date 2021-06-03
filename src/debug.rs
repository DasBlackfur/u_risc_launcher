use crate::print_help;
use std::fs::File;
use std::io::Read;
use u_risc_interpreter::Cpu;
use crate::devices::Terminal;

pub fn main(args: Vec<String>, mut file: File) {
    let mut bin: [u8; 65535] = [0; 65535];
    match file.read(&mut bin) {
        Ok(_) => {println!("Opened file successfully!")}
        Err(e) => {print_help(&format!("ERROR: error while reading specified file {}", e))}
    }
    let mut cpu = Cpu::new(bin, vec![]);
}