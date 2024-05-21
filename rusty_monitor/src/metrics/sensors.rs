use psutil::sensors;

use crate::psutil_error::metric_error::MetricsError;

#[derive(Debug)]
pub struct Temperature {
    pub current: f64,
    pub high: f64,
    pub critical: f64,
}

impl Temperature {
    fn new() -> Self {
        Temperature {
            current: 0.0,
            high: 0.0,
            critical: 0.0,
        }
    }
}


#[derive(Debug)]
pub struct Sensor {
    pub temperature: Temperature,
}

impl Sensor {
    pub fn new() -> Self {
        Sensor {
            temperature: Temperature::new(),
        }
    }

    pub fn get_sensor_temperature(&mut self) -> Result<(), MetricsError> {
        let temperatures = sensors::temperatures();
        // multiple temperature sensors
        for temp in temperatures.iter() {
            if let Ok(t) = temp {
                let current = t.current();
                let high = t.high();
                let critical = t.critical();
                self.temperature.current = current.celsius();
                
                println!("Current temperature: {:#?}°C", current);
            }
        }

        Ok(())
    }
}