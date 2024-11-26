use h3ist::{
    config::SystemConfig,
    controller::LiftController,
};

#[tokio::main]
async fn main() {
    println!("Smart Lift System Starting...");
    
    let config = SystemConfig::new(
        4,
        32,
        2.5,
        2.0,
        800.0, 
    );
    
    let mut controller = LiftController::new(config);
    controller.start().await;
}