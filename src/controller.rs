use crate::config::SystemConfig;
use crate::elevator::Elevator;
use crate::scheduler::Scheduler;
use crate::Request;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time;

pub struct LiftController {
    config: SystemConfig,
    elevators: Vec<Arc<Mutex<Elevator>>>,
    scheduler: Scheduler,
}

impl LiftController {
    pub fn new(config: SystemConfig) -> Self {
        let elevators: Vec<Arc<Mutex<Elevator>>> = (0..config.num_elevators)
            .map(|id| Arc::new(Mutex::new(Elevator::new(id, &config))))
            .collect();
        
        let scheduler = Scheduler::new(elevators.clone());
        
        Self {
            config,
            elevators,
            scheduler,
        }
    }

    pub async fn start(&mut self) {
        println!("Lift controller started with {} elevators", self.config.num_floors);
        println!("Building has {} floors (Ground floor = 1)", self.config.num_floors);

        // Create channels for request handling
        let (tx, mut rx) = mpsc::channel(100);
        let elevators = self.elevators.clone();
        let config = self.config.clone();

        // Spawn elevator update task
        let update_elevators = elevators.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(100));
            loop {
                interval.tick().await;
                for elevator in &update_elevators {
                    let mut elevator = elevator.lock().unwrap();
                    elevator.update(0.1); // 100ms = 0.1s
                }
            }
        });

        // Spawn request handling task
        let request_elevators = elevators.clone();
        tokio::spawn(async move {
            let mut scheduler = Scheduler::new(request_elevators);
            while let Some(request) = rx.recv().await {
                scheduler.add_request(request);
            }
        });

        // Main simulation loop
        loop {
            // Clear screen (ANSI escape code)
            print!("\x1B[2J\x1B[1;1H");
            
            println!("\nCurrent elevator status:");
            for elevator in &self.elevators {
                let elevator = elevator.lock().unwrap();
                println!("Elevator {} - Floor {:.1}, Status: {:?}, Door: {:?}", 
                    elevator.id, 
                    elevator.current_floor,
                    elevator.direction,
                    elevator.door_state
                );
            }
            
            println!("\nOptions:");
            println!("1. Request elevator (as passenger)");
            println!("2. Simulate random requests");
            println!("3. Exit");
            
            if tokio::time::timeout(Duration::from_millis(100), async {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                input
            }).await.is_ok() {
                let input = String::new();
                match input.trim() {
                    "1" => self.handle_passenger_request(tx.clone()).await,
                    "2" => self.simulate_random_requests(tx.clone(), &config).await,
                    "3" => break,
                    _ => println!("Invalid option"),
                }
            }
            
            // Small delay to prevent CPU overuse
            time::sleep(Duration::from_millis(100)).await;
        }
    }

    async fn handle_passenger_request(&self, tx: mpsc::Sender<Request>) {
        println!("Enter your current floor (1-{}):", self.config.num_floors);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let from_floor: u32 = match input.trim().parse() {
            Ok(n) if n >= 1 && n <= self.config.num_floors => n,
            _ => {
                println!("Invalid floor number");
                return;
            }
        };

        println!("Enter your destination floor (1-{}):", self.config.num_floors);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let to_floor: u32 = match input.trim().parse() {
            Ok(n) if n >= 1 && n <= self.config.num_floors && n != from_floor => n,
            _ => {
                println!("Invalid floor number");
                return;
            }
        };

        let request = Request {
            from_floor,
            to_floor,
            timestamp: Instant::now(),
            passenger_count: 1,
        };

        tx.send(request).await.unwrap();
        println!("Request added: Floor {} → {}", from_floor, to_floor);
    }

    async fn simulate_random_requests(&self, tx: mpsc::Sender<Request>, config: &SystemConfig) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        println!("How many random requests to simulate? (1-10):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        let count: u32 = match input.trim().parse() {
            Ok(n) if n >= 1 && n <= 10 => n,
            _ => {
                println!("Invalid number");
                return;
            }
        };

        for _ in 0..count {
            let from_floor = rng.gen_range(1..=config.num_floors);
            let to_floor = loop {
                let floor = rng.gen_range(1..=config.num_floors);
                if floor != from_floor {
                    break floor;
                }
            };

            let request = Request {
                from_floor,
                to_floor,
                timestamp: Instant::now(),
                passenger_count: rng.gen_range(1..=4),
            };

            tx.send(request.clone()).await.unwrap();
            println!("Added request: Floor {} → {} ({} passengers)", 
                request.from_floor, 
                request.to_floor,
                request.passenger_count
            );
            
            // Small delay between requests
            time::sleep(Duration::from_millis(500)).await;
        }
    }
}