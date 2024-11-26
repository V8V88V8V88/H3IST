use crate::config::SystemConfig;
use crate::{Direction, DoorState, Request};

#[derive(Debug)]
pub struct Elevator {
    pub id: usize,
    pub current_floor: f64,
    pub target_floor: Option<u32>,
    pub direction: Direction,
    pub passengers: Vec<Request>,
    pub speed: f64,
    pub acceleration: f64,
    pub load: f64,
    pub max_load: f64,
    pub door_state: DoorState,
}

impl Elevator {
    pub fn new(id: usize, config: &SystemConfig) -> Self {
        Self {
            id,
            current_floor: 1.0,
            target_floor: None,
            direction: Direction::Idle,
            passengers: Vec::new(),
            speed: 0.0,
            acceleration: config.acceleration_rate,
            load: 0.0,
            max_load: config.max_load,
            door_state: DoorState::Closed,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        match self.target_floor {
            Some(target) => {
                let target_pos = target as f64;
                let distance = target_pos - self.current_floor;
                
                self.direction = if distance > 0.01 {
                    Direction::Up
                } else if distance < -0.01 {
                    Direction::Down
                } else {
                    Direction::Idle
                };
                
                if distance.abs() < 0.01 {
                    self.arrive_at_floor();
                    return;
                }

                let stopping_distance = (self.speed * self.speed) / (2.0 * self.acceleration);
                let direction = distance.signum();
                
                if stopping_distance >= distance.abs() {
                    self.speed -= self.acceleration * direction * delta_time;
                } else {
                    self.speed += self.acceleration * direction * delta_time;
                }

                self.current_floor += self.speed * delta_time;
                
                self.current_floor = self.current_floor.max(1.0);
            }
            None => {
                self.speed = 0.0;
                self.direction = Direction::Idle;
            }
        }

        match self.door_state {
            DoorState::Opening => {
                self.door_state = DoorState::Open;
            }
            DoorState::Closing => {
                self.door_state = DoorState::Closed;
            }
            _ => {}
        }
    }

    fn arrive_at_floor(&mut self) {
        self.speed = 0.0;
        self.current_floor = self.target_floor.unwrap() as f64;
        self.target_floor = None;
        self.direction = Direction::Idle;
        self.door_state = DoorState::Opening;
        
        println!("Elevator {} arrived at floor {}", self.id, self.current_floor);
    }
}