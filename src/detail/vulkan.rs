extern crate winit;
extern crate vulkano;
extern crate log;

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
    instance: Arc<vulkano::instance::Instance>
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

        log::info!("==============================Initializing RustyBear-Engine==============================");

        let library = vulkano::library::VulkanLibrary::new()
            .unwrap_or_else(|err| panic!("Couldn't load Vulkan library: {:?}", err));

        let mut layers = vec!["VK_LAYER_KHRONOS_validation".to_string()];

        if VALIDATION_LAYERS_ENABLED
        {
            if !Self::validation_layers_available(library.clone(), layers.as_ref())
            {
                panic!("Validation layers are enabled, but not available!");
            }

            log::info!("Success! Loading validation layers...");
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

        return Instance { instance: instance.clone() };
    }

    fn validation_layers_available(library: Arc<vulkano::library::VulkanLibrary>, layers: &Vec<String>) -> bool
    {
        log::info!("Loading available Layers...");

        let sup_layers: Vec<String> = library.layer_properties().unwrap().map(|a| a.name().to_owned()).collect();

        for l in sup_layers.clone()
        {
            log::debug!("Available layer: {}", l);
        }

        return layers.iter().all(|layer| sup_layers.contains(&layer.to_string()));
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

