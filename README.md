Shoddy patchwork to "solve" a memory leak in explorer.exe

Runs a job every 30 minutes to check whether explorer.exe is taking up over a gigabyte of RAM, and if so, restarts it

To run it on stable I had to modify a file in sysinfo that used #[cfg] in an if, luckily there was only one
