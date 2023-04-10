
#[cfg_attr(target_os = "macos", path = "vulkan_core.rs")]
#[cfg_attr(target_os = "windows", path = "vulkan_core.rs")]
#[cfg_attr(target_os = "linux", path = "vulkan_core.rs")]
mod vulkan_core;
pub(crate) use vulkan_core::{Window, Instance, Version, DeviceInfo, Device, Swapchain};