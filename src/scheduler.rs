use std::sync::{Arc, Mutex};
use crate::elevator::Elevator;
use crate::{Request, Direction};
use crate::algorithms::{Assignment, OptimizedScan, SchedulingAlgorithm};

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
        println!("\nScheduler: Processing new request from floor {} to {}", 
            request.from_floor, request.to_floor);
        
        // Find the best elevator for this request
        let mut best_elevator = None;
        let mut min_cost = f64::MAX;

        for (i, elevator) in self.elevators.iter().enumerate() {
            let elevator = elevator.lock().unwrap();
            
            // Calculate cost based on distance and direction
            let cost = match elevator.target_floor {
                None => {
                    // Idle elevator - cost is just the distance
                    (elevator.current_floor - request.from_floor as f64).abs()
                }
                Some(target) => {
                    // Moving elevator - consider direction and current target
                    match elevator.direction {
                        Direction::Up if request.from_floor >= elevator.current_floor as u32 
                            && request.from_floor <= target => {
                            // Request is on the way up
                            (elevator.current_floor - request.from_floor as f64).abs()
                        }
                        Direction::Down if request.from_floor <= elevator.current_floor as u32 
                            && request.from_floor >= target => {
                            // Request is on the way down
                            (elevator.current_floor - request.from_floor as f64).abs()
                        }
                        _ => {
                            // Request requires a direction change
                            (elevator.current_floor - target as f64).abs() + 
                            (target as f64 - request.from_floor as f64).abs()
                        }
                    }
                }
            };

            if cost < min_cost {
                min_cost = cost;
                best_elevator = Some(i);
            }
        }

        // Assign the request to the best elevator
        if let Some(elevator_id) = best_elevator {
            let mut elevator = self.elevators[elevator_id].lock().unwrap();
            if elevator.target_floor.is_none() {
                println!("Assigning elevator {} to request (cost: {:.1})", elevator_id, min_cost);
                elevator.target_floor = Some(request.from_floor);
            }
        }

        self.requests.push(request);
    }
}