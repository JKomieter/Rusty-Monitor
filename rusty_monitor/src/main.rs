mod metrics;
mod psutil_error;

use metrics::cpu::CPU;
use metrics::disk::Disk;
use metrics::sensors::Sensor;

fn main() {
    let mut cpu_info = CPU::new();
    match cpu_info.get_cpu_usage() {
        Ok(()) => {
            println!("CPU: {}", cpu_info);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    let mut disk_info = Disk::new();
    match disk_info.get_disk_usage() {
        Ok(()) => {
            println!("Disk: {:?}", disk_info);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    
    let mut sensor_info = Sensor::new();
    #[cfg(any(target_os = "linux", target_os = "unix", target_os = "windows"))]
    match sensor_info.get_sensor_temperature() {
        Ok(()) => {
            println!("Sensor: {:?}", sensor_info);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

}
