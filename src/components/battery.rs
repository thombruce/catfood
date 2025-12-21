use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Battery {
    pub percentage: String,
    pub is_charging: bool,
    battery_manager: battery::Manager,
    battery: battery::Battery,
    last_update: Instant,
    update_interval: Duration,
}

impl Battery {
    pub fn new() -> color_eyre::Result<Self> {
        let manager = battery::Manager::new()?;
        let battery = match manager.batteries()?.next() {
            Some(Ok(battery)) => battery,
            Some(Err(e)) => {
                eprintln!("Unable to access battery information");
                return Err(e.into());
            }
            None => {
                eprintln!("Unable to find any batteries");
                return Err(std::io::Error::from(std::io::ErrorKind::NotFound).into());
            }
        };

        let is_charging = matches!(battery.state(), battery::State::Charging);

        Ok(Self {
            percentage: ((battery.state_of_charge().value * 100.0) as i32).to_string(),
            is_charging,
            battery_manager: manager,
            battery,
            last_update: Instant::now(),
            update_interval: Duration::from_secs(3),
        })
    }

    pub fn update(&mut self) -> color_eyre::Result<()> {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            self.battery_manager.refresh(&mut self.battery)?;
            self.percentage = ((self.battery.state_of_charge().value * 100.0) as i32).to_string();
            self.is_charging = matches!(self.battery.state(), battery::State::Charging);

            self.last_update = now;
        }
        Ok(())
    }
}
