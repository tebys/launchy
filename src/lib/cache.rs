use std::sync::mpsc;

use crate::lib::*;

pub fn cache_process(config: &Config) -> IconReceiver {
    let (tx, icon_rx) = mpsc::channel();
    for service in &config.services {
        if let Some(url) = service.icon_url.clone()
            && let Some(hashed_url) = service.hashed_icon_url.clone()
        {
            if let Some(path) = &config.cache_path
                && let Ok(bytes) = std::fs::read(format!("{}/{}", path, hashed_url))
            {
                send_icon(&tx, &bytes, hashed_url);
            } else {
                let icon_tx = tx.clone();
                let path = config.cache_path.clone();
                std::thread::spawn(move || {
                    if let Ok(response) = reqwest::blocking::get(&url)
                        && let Ok(bytes) = response.bytes()
                        && let Some(p) = path
                    {
                        let _ = std::fs::write(format!("{}/{}", p, hashed_url), &bytes); // Discard cache errors
                        send_icon(&icon_tx, &bytes, hashed_url);
                    }
                });
            }
        }
    }

    icon_rx
}

fn send_icon(icon_tx: &IconSender, bytes: &[u8], hashed_url: String) {
    if let Ok(image) = image::load_from_memory(&bytes) {
        let image = image.to_rgba8();
        let size = [image.width() as usize, image.height() as usize];
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &image.into_raw());
        let _ = icon_tx.send((hashed_url, color_image));
    }
}
