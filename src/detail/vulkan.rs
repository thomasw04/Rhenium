extern crate winit;
extern crate vulkano;
extern crate log;

use vulkano::{instance::debug::{DebugUtilsMessengerCreateInfo, DebugUtilsMessageSeverity, DebugUtilsMessageType, DebugUtilsMessenger}};
use winit::{dpi::LogicalSize, event::{Event, WindowEvent}};
use std::{sync::Arc};

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

