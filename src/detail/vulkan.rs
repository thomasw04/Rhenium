extern crate winit;
extern crate vulkano;
extern crate log;

use vulkano::{instance::debug::{DebugUtilsMessengerCreateInfo, DebugUtilsMessageSeverity, DebugUtilsMessageType, DebugUtilsMessenger}, device::{Properties, physical::PhysicalDeviceType, physical::PhysicalDevice, DeviceCreateInfo, QueueCreateInfo, Queue}, memory::MemoryProperties};
use winit::{dpi::LogicalSize, event::{Event, WindowEvent}};
use std::{sync::Arc, borrow::BorrowMut};

#[cfg(all(debug_assertions))]
const VALIDATION_LAYERS_ENABLED: bool = true;
#[cfg(not(debug_assertions))]
const VALIDATION_LAYERS_ENABLED: bool = false;

pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

pub struct Instance {
    instance: Arc<vulkano::instance::Instance>,
    debug: Option<DebugUtilsMessenger>,
}

#[derive(Default)]
pub struct DeviceInfo {
    index: usize,
    queue_index: usize,
    discrete: bool,
    memory: u32,
    score: u32,
}

pub struct Device {
    device: Arc<vulkano::device::Device>,
    info: DeviceInfo,
    queues: Vec<Arc<Queue>>,
}

pub struct Window {
    title: &'static str,
    width: u32,
    height: u32,
    window: winit::window::Window,
    event_loop: winit::event_loop::EventLoop<()>,
}

impl Instance {
    pub fn new(name: String, version: Version) -> Self {
        log::info!("Loading vulkan library...");

        let library = vulkano::library::VulkanLibrary::new()
            .unwrap_or_else(|err| panic!("Couldn't load Vulkan library: {:?}", err));

        let mut layers = vec!["VK_LAYER_KHRONOS_validation".to_string()];

        if VALIDATION_LAYERS_ENABLED
        {
            if !Self::validation_layers_available(&library, &layers)
            {
                log::error!("Validation layers are enabled, but not available. (Did you install the VulkanSDK?)");
                layers.clear();
            }
            else
            {
                log::info!("Found required layers. Installing...");
            }            
        }
        else
        {
            layers.clear();
        }

        let info = vulkano::instance::InstanceCreateInfo {
            application_name: Some(name),
            application_version: vulkano::Version { major: version.major, minor: version.minor, patch: version.patch },
            engine_name: Some("RustyBear-Engine".into()),
            engine_version: vulkano::Version {major: 1, minor: 0, patch: 0},

            //For now just enable all supported extensions. Should be changed.
            enabled_extensions: library.supported_extensions().clone(),
            enabled_layers: layers,
            .. Default::default()
        };

        log::info!("Loading vulkan instance...");

        let instance = vulkano::instance::Instance::new(library.clone(), info)
            .unwrap_or_else(|err| panic!("Couldn't create instance: {:?}", err));

        let callback = Self::create_debug_callback(&instance);

        return Instance { instance: instance.clone(), debug: callback};
    }

    fn validation_layers_available(library: &Arc<vulkano::library::VulkanLibrary>, layers: &Vec<String>) -> bool
    {
        log::info!("Loading available layers...");

        let sup_layers: Vec<String> = library.layer_properties().unwrap().map(|a| a.name().to_owned()).collect();

        for l in sup_layers.clone()
        {
            log::debug!("  Available layer: {}", l);
        }

        return layers.iter().all(|layer| sup_layers.contains(&layer.to_string()));
    }

    fn create_debug_callback(instance: &Arc<vulkano::instance::Instance>) -> Option<vulkano::instance::debug::DebugUtilsMessenger>
    {
        if !VALIDATION_LAYERS_ENABLED { return None; }

        let callback = unsafe {
            DebugUtilsMessenger::new(instance.to_owned(), 
                DebugUtilsMessengerCreateInfo 
                { 
                    message_severity: DebugUtilsMessageSeverity { error: true, warning: true, information: false, verbose: false, .. Default::default()},
                    message_type: DebugUtilsMessageType { general: true, validation: true, performance: true, .. Default::default()},
                    ..DebugUtilsMessengerCreateInfo::user_callback(Arc::new(|msg| 
                    {
                        let part = if msg.ty.general 
                        { 
                            "General"
                        }
                        else if msg.ty.performance
                        {
                            "Performance"
                        }
                        else if msg.ty.validation
                        {
                            "Validation"
                        }
                        else 
                        {
                            "Unknown"
                        };
                        

                        if msg.severity.error
                        {
                            log::error!("[{}] {}", part, msg.description);
                        }
                        else if msg.severity.warning
                        {
                            log::warn!("[{}] {}", part, msg.description);
                        }
                        else if msg.severity.information
                        {
                            log::info!("[{}] {}", part, msg.description);
                        }
                    }))
                },
            ).ok()
        };

        return callback;
    }

}

impl DeviceInfo {

    pub fn new(instance: &Instance) -> Self
    {
        let mut best_device = DeviceInfo {..Default::default()};

        for (pos, dev) in instance.instance.enumerate_physical_devices().unwrap().enumerate()
        {
            let mut device = DeviceInfo { index: pos, .. Default::default()};

            Self::compute_base_score(device.borrow_mut() ,dev.properties());
            Self::compute_memory_score(device.borrow_mut(), dev.memory_properties());
            Self::find_queue_families(device.borrow_mut(), dev);

            if device.score > best_device.score
            {
               best_device = device;
            }
        }

        return best_device;
    }

    fn compute_base_score(device: &mut DeviceInfo, props: &Properties)
    {
        //Very basic scoring function. Should be extended in the future. (score += 0 is kind of a placeholder)
        match props.device_type 
        {
            PhysicalDeviceType::DiscreteGpu => { device.score += 100; device.discrete = true; },
            PhysicalDeviceType::IntegratedGpu => { device.score += 10; device.discrete = false; },
            PhysicalDeviceType::VirtualGpu => { device.score += 1; device.discrete = false; },
            PhysicalDeviceType::Other => { device.score += 0; device.discrete = false; },
            _ => { device.score += 0; device.discrete = false; },
        }
    }

    fn compute_memory_score(device: &mut DeviceInfo, props: &MemoryProperties)
    {
        //Not implemented.
        device.memory = 0;
    }

    fn find_queue_families(device: &mut DeviceInfo, phys: Arc<PhysicalDevice>)
    {
        for (pos, family) in phys.queue_family_properties().iter().enumerate()
        {
            if family.queue_flags.graphics
            {
                device.queue_index = pos;
                return;
            }


        }
    }

}

impl Device {
    pub fn new(instance: &Instance, info: DeviceInfo) -> Self
    {
        let phys = instance.instance.enumerate_physical_devices().unwrap().nth(info.index).unwrap();

        let (device, queues) = vulkano::device::Device::new(phys,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index: info.queue_index as u32,
                ..Default::default()
            }],
            ..Default::default()
        }).expect("Failed to create device.");

        return Device { device: device, info: info, queues: queues.collect()}
    }
}

impl Window {

    pub fn new(title: &'static str, width: u32, height: u32) -> Self
    {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(f64::from(width), f64::from(height)))
            .build(&event_loop)
            .expect("Failed to create the window.");
        Window { title: title, width: width, height: height, window: window, event_loop: event_loop }
    }

    pub fn run<'a>(self, callback: impl Fn() + 'static)
    {
        self.event_loop.run(move |event, _, control_flow|
        {
            control_flow.set_poll();
            callback();

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    control_flow.set_exit();
                },
                _ => ()
            }
        });  
    }
}

