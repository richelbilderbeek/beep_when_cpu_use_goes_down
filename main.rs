use sysinfo::System;
use std::io::Write;

fn get_cpu_usages() -> Vec<f32> {
    let mut v: Vec<f32> = Vec::new();
    let mut sys = System::new();
    sys.refresh_cpu(); // Refreshing CPU information.
    for cpu in sys.cpus() {
        v.push(cpu.cpu_usage());
    }
    return v;

}

fn get_average_cpu_usage() -> f32 {
    let v = get_cpu_usages();
    v.iter().sum::<f32>() as f32 / v.len() as f32
}

fn main() {
    //let mut sys = System::new();
    loop {
        //sys.refresh_cpu(); // Refreshing CPU information.
        //for cpu in sys.cpus() {
        //    print!("{}% ", cpu.cpu_usage());
        //}
        println!("{}", get_average_cpu_usage());
        let _ = std::io::stdout().flush();
        // Sleeping to let time for the system to run for long
        // enough to have useful information.
        let wait_time = std::time::Duration::from_millis(1000);
        std::thread::sleep(wait_time);
    }
}
