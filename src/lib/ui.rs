use eframe::egui;
use egui::Ui;
use std::process::Command;

use crate::lib::{IconCache, MyApp, ServiceConfig};

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Revisar si hay iconos cargados en background
        while let Ok((url, icon)) = self.icon_rx.try_recv() {
            self.icon_cache.insert(url, icon);
        }

        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: egui::Color32::from_rgba_premultiplied(20, 20, 20, 200),
                inner_margin: egui::Margin::symmetric(10, 8),
                ..Default::default()
            })
            .show(ctx, |ui| {
                let response = ui.text_edit_singleline(&mut self.text_input);
                ui.memory_mut(|mem| mem.request_focus(response.id));

                ui.add_space(8.0);

                // Separar servicios por columna
                let left_services: Vec<_> = self
                    .services
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i % 2 == 0)
                    .collect();
                let right_services: Vec<_> = self
                    .services
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i % 2 == 1)
                    .collect();

                // Crear dos columnas
                ui.columns(2, |columns| {
                    // Columna izquierda (Ctrl+N)
                    columns[0].vertical(|ui| {
                        ui.heading("Ctrl");
                        ui.add_space(4.0);
                        draw_services(&self.icon_cache, ctx, &left_services, ui);
                    });

                    // Columna derecha (Alt+N)
                    columns[1].vertical(|ui| {
                        ui.heading("Alt");
                        ui.add_space(4.0);
                        draw_services(&self.icon_cache, ctx, &right_services, ui);
                    });
                });

                // Manejar atajos (Ctrl+N, Alt+N)
                handle_shortcuts(ctx, &left_services, &right_services);

                // Cerrar aplicación con Esc
                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }

                // Ejecutar comando al presionar Enter
                if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if !self.text_input.trim().is_empty() {
                        let param = if !self.text_input.trim().starts_with("http://")
                            && !self.text_input.trim().starts_with("https://")
                        {
                            format!("https://{}", self.text_input.trim())
                        } else {
                            self.text_input.trim().to_string()
                        };

                        std::thread::spawn(move || {
                            if let Err(e) = Command::new("omarchy-launch-webapp").arg(param).spawn()
                            {
                                eprintln!("Error al ejecutar omarchy-launch-webapp: {}", e);
                            }
                        });
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
            });
    }
}

fn handle_shortcuts(
    ctx: &egui::Context,
    left_services: &Vec<(usize, &ServiceConfig)>,
    right_services: &Vec<(usize, &ServiceConfig)>,
) {
    if let Some((input, services)) = ctx.input(|i| {
        if i.modifiers.ctrl {
            Some((i.clone(), left_services))
        } else if i.modifiers.alt {
            Some((i.clone(), right_services))
        } else {
            None
        }
    }) {
        let service_url = if input.key_pressed(egui::Key::Num1) && services.len() > 0 {
            Some(services[0].1.url.clone())
        } else if input.key_pressed(egui::Key::Num2) && services.len() > 1 {
            Some(services[1].1.url.clone())
        } else if input.key_pressed(egui::Key::Num3) && services.len() > 2 {
            Some(services[2].1.url.clone())
        } else if input.key_pressed(egui::Key::Num4) && services.len() > 3 {
            Some(services[3].1.url.clone())
        } else if input.key_pressed(egui::Key::Num5) && services.len() > 4 {
            Some(services[4].1.url.clone())
        } else if input.key_pressed(egui::Key::Num6) && services.len() > 5 {
            Some(services[5].1.url.clone())
        } else if input.key_pressed(egui::Key::Num7) && services.len() > 6 {
            Some(services[6].1.url.clone())
        } else if input.key_pressed(egui::Key::Num8) && services.len() > 7 {
            Some(services[7].1.url.clone())
        } else if input.key_pressed(egui::Key::Num9) && services.len() > 8 {
            Some(services[8].1.url.clone())
        } else {
            None
        };

        if let Some(url) = service_url {
            launch_service(ctx, url);
        }
    }
}

fn launch_service(ctx: &egui::Context, url: String) {
    std::thread::spawn(move || {
        if let Err(e) = Command::new("omarchy-launch-webapp").arg(url).spawn() {
            eprintln!("Error al ejecutar omarchy-launch-webapp: {}", e);
        }
    });
    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
}

fn draw_services(
    icon_cache: &IconCache,
    ctx: &egui::Context,
    services: &Vec<(usize, &ServiceConfig)>,
    ui: &mut Ui,
) {
    for (local_index, (global_index, service)) in services.iter().enumerate() {
        ui.horizontal(|ui| {
            let mut icon_loaded = false;

            if let Some(url) = &service.hashed_icon_url
                && let Some(icon) = icon_cache.get(url)
            {
                let texture_id = ui.ctx().load_texture(
                    url,
                    egui::ColorImage::clone(icon),
                    egui::TextureOptions::default(),
                );
                ui.image(egui::load::SizedTexture::new(
                    texture_id.id(),
                    egui::Vec2::new(16.0, 16.0),
                ));
                icon_loaded = true;
            }

            if !icon_loaded {
                let rect = egui::Rect::from_min_size(ui.cursor().min, egui::Vec2::new(16.0, 16.0));
                let color = match *global_index {
                    0 => egui::Color32::RED,
                    1 => egui::Color32::BLUE,
                    2 => egui::Color32::from_rgb(66, 133, 244),
                    _ => egui::Color32::GRAY,
                };
                ui.painter().circle_filled(rect.center(), 8.0, color);
                ui.advance_cursor_after_rect(rect);
            }

            let label = format!("{}. {}", local_index + 1, service.name);
            if ui.button(label).clicked() {
                let param = service.url.clone();
                std::thread::spawn(move || {
                    if let Err(e) = Command::new("omarchy-launch-webapp").arg(param).spawn() {
                        eprintln!("Error al ejecutar omarchy-launch-webapp: {}", e);
                    }
                });
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }
}
