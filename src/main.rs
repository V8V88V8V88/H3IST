use h3ist::{
    config::SystemConfig,
    controller::LiftController,
};

#[tokio::main]
async fn main() {
    println!("Smart Lift System Starting...");
    
    let config = SystemConfig::new(
        4,      // number of elevators
        32,     // number of floors
        2.5,    // acceleration rate (m/s²)
        2.0,    // max speed (m/s)
        800.0,  // max load (kg)
    );
    
    let mut controller = LiftController::new(config);
    controller.start().await;
}