use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::elevator::Elevator;
use crate::Request;

#[derive(Debug)]
pub struct Assignment {
    pub elevator_id: usize,
    pub requests: Vec<Request>,
    pub cost: f64,
}

pub trait SchedulingAlgorithm: Send + Sync {
    fn compute_assignments(
        &self,
        requests: &[Request],
        elevators: &[Arc<Mutex<Elevator>>]
    ) -> Vec<Assignment>;
}

pub struct OptimizedScan {
    look_ahead_window: Duration,
    energy_weight: f64,
    wait_time_weight: f64,
}

impl OptimizedScan {
    pub fn new() -> Self {
        Self {
            look_ahead_window: Duration::from_secs(5),
            energy_weight: 0.5,
            wait_time_weight: 0.5,
        }
    }

    fn calculate_energy_cost(&self, _elevator: &Elevator, _up_requests: &[&Request], _down_requests: &[&Request]) -> f64 {
        0.0
    }

    fn calculate_wait_cost(&self, _elevator: &Elevator, _up_requests: &[&Request], _down_requests: &[&Request]) -> f64 {
        0.0
    }

    fn select_optimal_requests(&self, _elevator: &Elevator, _up_requests: &[&Request], _down_requests: &[&Request]) -> Vec<Request> {
        Vec::new()
    }
}

impl SchedulingAlgorithm for OptimizedScan {
    fn compute_assignments(
        &self,
        requests: &[Request],
        elevators: &[Arc<Mutex<Elevator>>]
    ) -> Vec<Assignment> {
        let mut assignments = Vec::new();
        
        let mut up_requests = Vec::new();
        let mut down_requests = Vec::new();
        
        for request in requests {
            if request.to_floor > request.from_floor {
                up_requests.push(request);
            } else {
                down_requests.push(request);
            }
        }
        
        up_requests.sort_by_key(|r| r.from_floor);
        down_requests.sort_by_key(|r| std::cmp::Reverse(r.from_floor));
        
        for elevator in elevators {
            let elevator = elevator.lock().unwrap();
            
            let energy_cost = self.calculate_energy_cost(&elevator, &up_requests, &down_requests);
            
            let wait_cost = self.calculate_wait_cost(&elevator, &up_requests, &down_requests);
            
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