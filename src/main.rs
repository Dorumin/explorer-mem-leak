// Clippy settings to help my newbie Rust brain
#![deny(clippy::all)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]

// Don't show a command prompt
#![windows_subsystem = "windows"]

use std::sync::Arc;
use std::thread;
use std::time::{Instant};
use std::process::Command;
use std::sync::atomic::{Ordering, AtomicBool};
use core_extensions::ToTime;
use sysinfo::{Process, ProcessExt, System, SystemExt, Signal};

const MAX_MEMORY_IN_MB: u64 = 2000;
const TIME_BETWEEN_CHECKS: u64 = 60000;
const CHECK_INTERVAL: u64 = 500;

fn find_processes_by_name<'a>(system: &'a System, name: &'a str) -> impl Iterator<Item=&'a Process> {
    system
        .get_processes()
        .values()
        .filter(move |p| p.name() == name)
}

fn process_memory_in_mb(process: &Process) -> u64 {
    process.memory() * 1000 / 1024 / 1024
}

fn restart_process(process: &Process) {
    let args = process.cmd();
    process.kill(Signal::Kill);

    let mut process = Command::new(&args[0]);
    
    for arg in args.iter().skip(1) {
        process.arg(arg);
    }

    process.spawn().unwrap();

    drop(process); // Unlink
}

fn main() {
    let killed = Arc::new(AtomicBool::new(false));
    let killed2 = Arc::clone(&killed);

    let handle = thread::spawn(move || {
        let mut last_check = Instant::now();

        loop {
            if killed2.load(Ordering::SeqCst) {
                break;
            }

            if last_check.elapsed().as_millis() > TIME_BETWEEN_CHECKS.into() {
                last_check = Instant::now();

                for process in find_processes_by_name(&System::new_all(), "explorer.exe") {
                    println!("-> Found {name:?} process using {mem}mb of memory",
                        name = process.name(),
                        mem = process_memory_in_mb(process)
                    );

                    if process_memory_in_mb(process) > MAX_MEMORY_IN_MB {
                        restart_process(process);
                    }
                }
            }
    
            thread::sleep(CHECK_INTERVAL.milliseconds());
        }
    });

    ctrlc::set_handler(move || {
        for process in find_processes_by_name(&System::new_all(), "Taskmgr.exe") {
            process.kill(Signal::Kill);
        }

        killed.store(true, Ordering::SeqCst);
    })
    .expect("whoops the Ctrl+C handler failed to be set");

    handle.join().unwrap();
}
