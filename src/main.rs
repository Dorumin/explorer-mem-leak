#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![windows_subsystem = "windows"]

use std::{
    thread,
    time::Duration,
    process::Command
};
use sysinfo::{Process, ProcessExt, System, SystemExt, Signal};

fn get_explorers(system: &'_ System) -> impl Iterator<Item=&'_ Process> {
    system
        .get_processes()
        .values()
        .filter(|p| p.name() == "explorer.exe")
}

fn process_memory_in_mb(process: &Process) -> u64 {
    process.memory() * 1000 / 1024 / 1024
}

fn restart_process(process: &Process) {
    let path = process.exe();
    process.kill(Signal::Kill);

    Command::new(path).spawn().unwrap();
}

fn main() {
    loop {
        for process in get_explorers(&System::new_all()) {
            if process_memory_in_mb(process) > 900 {
                restart_process(process);
            }
        }

        thread::sleep(Duration::from_secs(60 * 30));
    }
}
