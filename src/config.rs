#[derive(Clone)]
pub struct SystemConfig {
    pub num_elevators: usize,
    pub num_floors: u32,
    pub acceleration_rate: f64,
    pub max_speed: f64,
    pub max_load: f64,
}

impl SystemConfig {
    pub fn new(
        num_elevators: usize,
        num_floors: u32,
        acceleration_rate: f64,
        max_speed: f64,
        max_load: f64,
    ) -> Self {
        Self {
            num_elevators,
            num_floors,
            acceleration_rate,
            max_speed,
            max_load,
        }
    }
}