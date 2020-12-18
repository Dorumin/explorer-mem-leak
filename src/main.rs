// Clippy settings to help my newbie Rust brain
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![allow(clippy::unseparated_literal_suffix)]

// Don't show a command prompt
#![windows_subsystem = "windows"]

use std::{
    thread,
    time::Duration,
    process::Command
};
use sysinfo::{Process, ProcessExt, System, SystemExt, Signal};

const MAX_MEMORY_IN_MB: u64 = 500u64;

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
            if process_memory_in_mb(process) > MAX_MEMORY_IN_MB {
                restart_process(process);
            }
        }

        thread::sleep(Duration::from_secs(60 * 30));
    }
}
