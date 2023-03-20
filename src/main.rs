extern crate log;

mod detail;
mod logging;

use detail::vulkan::{Window, Instance, Version};

use crate::detail::vulkan::{DeviceInfo, Device};

fn main() {
    logging::init();

    println!();
    log::info!("Initializing RustyBear-Engine. Stay tuned.");

    let instance = Instance::new(String::from("Luu's Cringe Adventure"), Version{major: 1, minor: 0, patch: 0});
    let deviceInfo = DeviceInfo::new(&instance);
    let device = Device::new(&instance, deviceInfo);

    let window = Window::new("RustyBear-Engine", 512, 512);
    window.run( || {});
}
