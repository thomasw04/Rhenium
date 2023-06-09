mod rhenium;
use rhenium::*;
mod logging;

use std::env;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    logging::init();

    println!();
    log::info!("Initializing RustyBear-Engine. Stay tuned.");

    let instance = Instance::new(String::from("Luu's Cringe Adventure"), Version{major: 1, minor: 0, patch: 0});
    let device_info = DeviceInfo::new(&instance);

    let device = Device::new(&instance, device_info);
    let window = Window::new(&instance, "RustyBear-Engine", 512, 512);
    let _swapchain = Swapchain::new(&device, &window);
    
    window.run( || {});
}
