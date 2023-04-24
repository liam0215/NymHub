mod logging;
<<<<<<< HEAD
<<<<<<< HEAD
mod parser;
mod command;
=======
>>>>>>> added logging and removed metrics
=======
use log::{info, warn, error};

>>>>>>> added logging
fn main() {
    logging::init_logger();
    println!("Hello, world!");
    info!("test1");
    warn!("test2");
    error!("test3");
}
