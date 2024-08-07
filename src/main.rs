#![windows_subsystem = "windows"]
use sysinfo::{Process, ProcessesToUpdate, System};

fn main() {
    unsafe { winapi::um::wincon::FreeConsole() };

    let keywords = ["GameManagerService.exe", "Razer", "Rz", "RZ"];

    let mut sys = System::new_all();
    for _ in 0..2 {
        sys.refresh_processes(ProcessesToUpdate::All);
        for (_pid, process) in sys.processes() {
            if keywords
                .iter()
                .any(|&keyword| process.name().to_str().unwrap().starts_with(keyword))
            {
                println!("Killing {:?}", &process.name());
                Process::kill(&process);
            }
        }
    }
}

#[test]
fn search() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let mut sys = sysinfo::System::new();
    sys.refresh_processes(ProcessesToUpdate::All);
    for (_pid, process) in sys.processes() {
        if process.name().to_str().unwrap().contains(input) {
            println!("{}", process.name().to_str().unwrap());
        }
    }
}
