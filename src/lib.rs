pub mod config;
pub mod controller;
pub mod elevator;
pub mod scheduler;
pub mod algorithms;

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

#[derive(Debug, Clone)]
pub enum DoorState {
    Open,
    Closed,
    Opening,
    Closing,
}