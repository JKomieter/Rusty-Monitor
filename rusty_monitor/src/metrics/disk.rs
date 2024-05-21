use psutil::disk;

use crate::psutil_error::metric_error::MetricsError;

#[derive(Debug)]
pub struct Usage {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub percentage: f32,
}

impl Usage {
    fn new() -> Self {
        Usage {
            total: 0,
            used: 0,
            free: 0,
            percentage: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct Disk {
    disk_usage: Usage,
}

impl Disk {
    pub fn new() -> Self {
        Disk {
            disk_usage: Usage::new(),
        }
    }

    pub fn get_disk_usage(&mut self) -> Result<(), MetricsError> {
        let usage = disk::disk_usage("/")?;
        self.disk_usage.total = usage.total();
        self.disk_usage.used = usage.used();
        self.disk_usage.free = usage.free();
        self.disk_usage.percentage = usage.percent();

        Ok(())
    }
}
