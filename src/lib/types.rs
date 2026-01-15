use eframe::egui;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::mpsc;

pub type IconReceiver = mpsc::Receiver<(String, egui::ColorImage)>;
pub type IconSender = mpsc::Sender<(String, egui::ColorImage)>;
pub type IconCache = HashMap<String, egui::ColorImage>;

pub struct MyApp {
    pub text_input: String,
    pub services: Vec<ServiceConfig>,
    pub icon_cache: IconCache,
    pub icon_rx: IconReceiver,
}

#[derive(Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub url: String,
    pub icon_url: Option<String>,
    pub hashed_icon_url: Option<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub services: Vec<ServiceConfig>,
    pub cache_path: Option<String>,
}

impl MyApp {
    pub fn new(config: Config, icon_rx: IconReceiver) -> Self {
        Self {
            text_input: String::new(),
            services: config.services,
            icon_cache: HashMap::new(),
            icon_rx,
        }
    }
}
