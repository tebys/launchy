use crate::lib::*;
use sha2::{Digest, Sha256};

pub fn startup() -> anyhow::Result<Config> {
    let home_dir = std::env::var("HOME")?;
    let conf_path = format!("{}/.config/launchy/conf.toml", home_dir);
    let conf_content = std::fs::read_to_string(conf_path)?;
    let mut parsed_conf: Config = toml::from_str(&conf_content)?;

    // Procesar hashed URLs
    for service in parsed_conf.services.iter_mut() {
        service.hashed_icon_url = service.icon_url.as_ref().map(|url| {
            let hashed = Sha256::digest(url);
            format!("{:x}", hashed)
        });
    }

    let cache_path = format!("{}/.cache/launchy/", home_dir);
    std::fs::create_dir_all(&cache_path)?;
    parsed_conf.cache_path = Some(cache_path);
    Ok(parsed_conf)
}
