// src/lib.rs
pub mod config;
pub mod controller;
pub mod elevator;
pub mod scheduler;
pub mod algorithms;

use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Idle,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub from_floor: u32,
    pub to_floor: u32,
    pub timestamp: std::time::Instant,
    pub passenger_count: u8,
}

// src/elevator.rs
pub struct Elevator {
    id: usize,
    current_floor: f64,
    target_floor: Option<u32>,
    direction: Direction,
    passengers: Vec<Request>,
    speed: f64,
    acceleration: f64,
    load: f64,
    max_load: f64,
    door_state: DoorState,
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
                
                if distance.abs() < 0.01 {
                    self.arrive_at_floor();
                    return;
                }

                // Calculate optimal speed profile
                let stopping_distance = (self.speed * self.speed) / (2.0 * self.acceleration);
                let direction = distance.signum();
                
                if stopping_distance >= distance.abs() {
                    // Decelerate
                    self.speed -= self.acceleration * direction * delta_time;
                } else {
                    // Accelerate
                    self.speed += self.acceleration * direction * delta_time;
                }

                self.current_floor += self.speed * delta_time;
            }
            None => {
                self.speed = 0.0;
            }
        }
    }

    fn arrive_at_floor(&mut self) {
        self.speed = 0.0;
        self.current_floor = self.target_floor.unwrap() as f64;
        self.target_floor = None;
        self.door_state = DoorState::Opening;
    }
}

// src/scheduler.rs
pub struct Scheduler {
    requests: Vec<Request>,
    elevators: Vec<Arc<Mutex<Elevator>>>,
    algorithm: Box<dyn SchedulingAlgorithm>,
}

impl Scheduler {
    pub fn new(elevators: Vec<Arc<Mutex<Elevator>>>) -> Self {
        Self {
            requests: Vec::new(),
            elevators,
            algorithm: Box::new(OptimizedScan::new()),
        }
    }

    pub fn add_request(&mut self, request: Request) {
        self.requests.push(request);
        self.optimize_assignments();
    }

    fn optimize_assignments(&mut self) {
        let assignments = self.algorithm.compute_assignments(
            &self.requests,
            &self.elevators
        );
        
        self.apply_assignments(assignments);
    }
}

// src/algorithms/mod.rs
pub trait SchedulingAlgorithm: Send + Sync {
    fn compute_assignments(
        &self,
        requests: &[Request],
        elevators: &[Arc<Mutex<Elevator>>]
    ) -> Vec<Assignment>;
}

pub struct OptimizedScan {
    look_ahead_window: std::time::Duration,
    energy_weight: f64,
    wait_time_weight: f64,
}

impl SchedulingAlgorithm for OptimizedScan {
    fn compute_assignments(
        &self,
        requests: &[Request],
        elevators: &[Arc<Mutex<Elevator>>]
    ) -> Vec<Assignment> {
        let mut assignments = Vec::new();
        
        // Group requests by direction and proximity
        let mut up_requests = Vec::new();
        let mut down_requests = Vec::new();
        
        for request in requests {
            if request.to_floor > request.from_floor {
                up_requests.push(request);
            } else {
                down_requests.push(request);
            }
        }
        
        // Sort requests by floor number
        up_requests.sort_by_key(|r| r.from_floor);
        down_requests.sort_by_key(|r| std::cmp::Reverse(r.from_floor));
        
        // Calculate costs for each elevator
        for elevator in elevators {
            let elevator = elevator.lock().unwrap();
            
            // Calculate energy cost
            let energy_cost = self.calculate_energy_cost(&elevator, &up_requests, &down_requests);
            
            // Calculate wait time cost
            let wait_cost = self.calculate_wait_cost(&elevator, &up_requests, &down_requests);
            
            // Combine costs using weights
            let total_cost = self.energy_weight * energy_cost + self.wait_time_weight * wait_cost;
            
            assignments.push(Assignment {
                elevator_id: elevator.id,
                requests: self.select_optimal_requests(&elevator, &up_requests, &down_requests),
                cost: total_cost,
            });
        }
        
        assignments
    }
}