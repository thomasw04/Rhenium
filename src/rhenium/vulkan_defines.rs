pub(super) use vulkano::Version;
pub(super) use vulkano::library::VulkanLibrary;
pub(super) use vulkano::image::{ImageUsage,SwapchainImage};
pub(super) use vulkano::instance::{Instance, InstanceCreateInfo};
pub(super) use vulkano::instance::debug::{DebugUtilsMessengerCreateInfo, DebugUtilsMessageSeverity, DebugUtilsMessageType, DebugUtilsMessenger};
pub(super) use vulkano::device::{Device, Properties, physical::PhysicalDeviceType, physical::PhysicalDevice, DeviceCreateInfo, QueueCreateInfo, Queue, QueueFlags, DeviceExtensions};
pub(super) use vulkano::memory::MemoryProperties;
pub(super) use vulkano::swapchain::{Swapchain, Surface, SwapchainCreateInfo};

pub(super) use winit::{dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::EventLoop, window::WindowBuilder};

pub(super) use vulkano_win::VkSurfaceBuild;
pub(super) use vulkano_win::required_extensions;


#[cfg(all(debug_assertions))]
pub const VALIDATION_LAYERS_ENABLED: bool = true;
#[cfg(not(debug_assertions))]
pub const VALIDATION_LAYERS_ENABLED: bool = false;