use sysinfo::System;

fn main() {
    let mut sys = System::new();
    loop {
        sys.refresh_cpu(); // Refreshing CPU information.
        for cpu in sys.cpus() {
        print!("{}% ", cpu.cpu_usage());
        }
        // Sleeping to let time for the system to run for long
        // enough to have useful information.
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}
