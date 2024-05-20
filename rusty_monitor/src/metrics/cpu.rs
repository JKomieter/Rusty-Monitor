use psutil::cpu::{self, os::unix::CpuTimesExt};

use crate::psutil_error::metric_error::MetricsError;


#[derive(Debug)]
pub struct Usage {
    pub user: f32,
    pub system: f32,
    pub idle: f32,
    pub nice: f32,
}

#[derive(Debug)]
pub struct CPU {
    pub per_core_usage: Vec<Usage>,
    pub total_usage: Usage,
    pub cpu_clock_speed: f32,
    pub temperature: f32,
    pub cpu_usage_percent: f64,
    pub context_switches: u64,
    pub interrupts: u64,
}

impl Usage {
    fn new() -> Self {
        Usage {
            user: 0.0,
            system: 0.0,
            idle: 0.0,
            nice: 0.0,
        }
    }
}

impl std::fmt::Display for Usage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User: {:.2} \n
        System: {:.2} \n
        Idle: {:.2} \n
        Nice: {:.2} \n
        ", self.user, self.system, self.idle, self.nice)
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            per_core_usage: Vec::new(),
            total_usage: Usage::new(),
            cpu_clock_speed: 0.0,
            cpu_usage_percent: 0.0,
            temperature: 0.0,
            context_switches: 0,
            interrupts: 0,
        }
    }

    pub fn get_cpu_usage(&mut self) -> Result<(), MetricsError> {
        let cpu_times_percpu = cpu::cpu_times_percpu()?;

        let mut total_user_time = 0.0;
        let mut total_system_time = 0.0;
        let mut total_idle_time = 0.0;
        let mut total_nice_time = 0.0;

        for cpu_times in &cpu_times_percpu {
            total_user_time += cpu_times.user().as_secs_f64();
            total_system_time += cpu_times.system().as_secs_f64();
            total_idle_time += cpu_times.idle().as_secs_f64();
            total_nice_time += cpu_times.nice().as_secs_f64();
        }

        let total_time = total_user_time + total_system_time + total_idle_time + total_nice_time;
        self.total_usage = Usage {
            user: total_user_time as f32,
            system: total_system_time as f32,
            idle: total_idle_time as f32,
            nice: total_nice_time as f32,
        };
        let busy_time = total_time - total_idle_time;
        let cpu_usage_percent = (busy_time / total_time) * 100.0;

        self.cpu_usage_percent = cpu_usage_percent;
        self.per_core_usage = cpu_times_percpu.iter().map(|cpu_times| Usage {
            user: cpu_times.user().as_secs_f32(),
            system: cpu_times.system().as_secs_f32(),
            idle: cpu_times.idle().as_secs_f32(),
            nice: cpu_times.nice().as_secs_f32(),
        }).collect();

        Ok(())
    }
}

impl std::fmt::Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CPU Usage: {:.2}% \n
        Total Usage: \n {} \n
        ", self.cpu_usage_percent,
        self.total_usage)
    }
}