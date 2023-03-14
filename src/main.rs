extern crate log;

mod detail;
mod logging;

use detail::vulkan::{Window, Instance, Version};

fn main() {
    logging::init();

    println!();
    log::info!("Initializing RustyBear-Engine. Stay tuned.");

    let instance = Instance::new(String::from("Luu's Cringe Adventure"), Version{major: 1, minor: 0, patch: 0});
    let window = Window::new("RustyBear-Engine", 512, 512);
    window.run( || {});
}
