mod metrics;
mod psutil_error;

use metrics::cpu::CPU;

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

}
