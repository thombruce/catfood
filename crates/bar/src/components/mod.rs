use serde::de::DeserializeOwned;

pub mod battery;
pub mod brightness;
pub mod cpu;
pub mod error_icon;
pub mod left_bar;
pub mod middle_bar;
pub mod ram;
pub mod right_bar;
pub mod separator;
pub mod space;
pub mod temperature;
pub mod time;
pub mod volume;
pub mod weather;
pub mod wifi;
pub mod windows;
pub mod workspaces;

pub trait ConfigurableComponent {
    type Config: DeserializeOwned;

    fn from_config(config: Self::Config) -> color_eyre::Result<Self>
    where
        Self: Sized;
}

/// Macro to register configurable components with the registry
/// This macro automatically generates the factory function and registration code
macro_rules! register_configurable_components {
    ($registry:expr, $($component:ident => $component_name:literal),+ $(,)?) => {
        $(
            $registry.register_component::<$component>($component_name);
        )+
    };
}

/// Macro to define all configurable components in one place
/// Usage: define_configurable_components!(Wifi => "wifi");
macro_rules! define_configurable_components {
    ($($component_type:ident => $component_name:literal),+ $(,)?) => {
        // Generate From implementations for each component
        $(
            impl From<$component_type> for crate::component_manager::Component {
                fn from(component: $component_type) -> Self {
                    crate::component_manager::Component::$component_type(component)
                }
            }
        )+

        // Helper function to register all components
        pub fn register_all_configurable_components(registry: &mut ConfigurableComponentRegistry) {
            register_configurable_components!(registry, $($component_type => $component_name),+);
        }
    };
}

// Define all configurable components here
// This single declaration handles:
// 1. From trait implementations 
// 2. Registration function generation
//
// To add a new configurable component:
// 1. Create a Config struct with serde deserialization
// 2. Implement ConfigurableComponent trait for your component  
// 3. Add the component to this macro
// 4. That's it! The macro handles all the boilerplate
define_configurable_components!(
    Wifi => "wifi",
    // Future components can be added here:
    // Battery => "battery",  // Uncomment to make battery configurable
    // Cpu => "cpu",          // Uncomment to make cpu configurable
    // Ram => "ram",          // Uncomment to make ram configurable
);

type ComponentFactory = std::boxed::Box<
    dyn Fn(&serde_json::Value) -> Option<color_eyre::Result<crate::component_manager::Component>>,
>;

pub struct ConfigurableComponentRegistry {
    factories: std::collections::HashMap<String, ComponentFactory>,
}

impl ConfigurableComponentRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            factories: std::collections::HashMap::new(),
        };

        // Auto-register all built-in configurable components using the macro-generated function
        // This makes it easy to add new configurable components - just add them to the 
        // define_configurable_components! macro above
        register_all_configurable_components(&mut registry);

        registry
    }

    /// Generic method to register any configurable component
    /// This allows runtime registration of new configurable components (for plugins)
    pub fn register_component<T>(&mut self, name: &str)
    where
        T: ConfigurableComponent + Into<crate::component_manager::Component> + 'static,
        T::Config: serde::de::DeserializeOwned,
    {
        let factory: ComponentFactory = std::boxed::Box::new(move |value: &serde_json::Value| {
            if let Ok(config) = serde_json::from_value::<T::Config>(value.clone())
                && let Ok(component) = T::from_config(config)
            {
                return Some(Ok(component.into()));
            }
            None
        });
        self.factories.insert(name.to_string(), factory);
    }

    pub fn try_create(
        &self,
        component_name: &str,
        value: &serde_json::Value,
    ) -> Option<color_eyre::Result<crate::component_manager::Component>> {
        if let Some(factory) = self.factories.get(component_name) {
            factory(value)
        } else {
            None
        }
    }

    pub fn extract_component_name(value: &serde_json::Value) -> String {
        if let Some(obj) = value.as_object()
            && let Some(component) = obj.get("component").and_then(|c| c.as_str())
        {
            return component.to_string();
        }
        "unknown".to_string()
    }
}

pub use battery::Battery;
pub use brightness::Brightness;
pub use cpu::Cpu;
pub use error_icon::ErrorIcon;
pub use left_bar::LeftBar;
pub use middle_bar::MiddleBar;
pub use ram::Ram;
pub use right_bar::RightBar;
pub use separator::Separator;
pub use space::Space;
pub use temperature::Temperature;
pub use time::Time;
pub use volume::Volume;
pub use weather::Weather;
pub use wifi::Wifi;
pub use windows::Windows;
pub use workspaces::Workspaces;
