use crate::icon_cache::{Icon, IconCache};
use eframe::egui;
use egui::Vec2;
use tracing::info;

pub struct IconViewerApp {
    icons: IconCache,
    scroll_area_id: egui::Id,
    dark_mode: bool,
    icon_size: f32,
    icon_size_options: Vec<(String, f32)>,
}

impl IconViewerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        info!("Initializing IconViewerApp");

        let icons = IconCache::new(&cc.egui_ctx);

        let mut app = Self {
            icons,
            scroll_area_id: egui::Id::new("icon_scroll"),
            dark_mode: false,
            icon_size: 64.0,
            icon_size_options: vec![
                ("Small (32px)".to_string(), 32.0),
                ("Medium (64px)".to_string(), 64.0),
                ("Large (96px)".to_string(), 96.0),
                ("Extra Large (128px)".to_string(), 128.0),
            ],
        };

        info!(
            "IconViewerApp initialization complete with {} icons",
            app.icons.len()
        );

        app
    }

    fn render_top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("{} icons loaded", self.icons.len()));
                ui.checkbox(&mut self.dark_mode, "Dark Mode");

                ui.separator();

                ui.label("Icon Size:");
                egui::ComboBox::from_id_salt("icon_size_combo")
                    .selected_text(
                        self.icon_size_options
                            .iter()
                            .find(|(_, size)| *size == self.icon_size)
                            .map(|(name, _)| name.as_str())
                            .unwrap_or("Medium (64px)"),
                    )
                    .show_ui(ui, |ui| {
                        for (name, size) in &self.icon_size_options {
                            ui.selectable_value(&mut self.icon_size, *size, name);
                        }
                    });
            });
        });
    }

    fn render_main_panel(&mut self, ctx: &egui::Context) -> egui::InnerResponse<()> {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Program Icons");
            ui.separator();

            egui::ScrollArea::vertical()
                .id_salt(self.scroll_area_id)
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        let available_width = ui.available_width();
                        let icon_size = self.icon_size;
                        let spacing = 10.0;
                        let icons_per_row =
                            ((available_width - spacing) / (icon_size + spacing)).floor() as usize;
                        let icons_per_row = icons_per_row.max(1);

                        for chunk in self.icons.chunks(icons_per_row) {
                            ui.horizontal(|ui| {
                                for icon in chunk {
                                    render_icon(icon_size, spacing, ui, icon);
                                }
                            });
                        }
                    });
                });
        })
    }
}

fn render_icon(icon_size: f32, spacing: f32, ui: &mut egui::Ui, icon: &Icon) {
    // TODO is it possible to set hover text on a vertical?
    ui.vertical(|ui| {
        ui.set_width(icon_size + spacing);

        match &icon {
            Icon::Texture {
                path,
                name,
                texture,
            } => {
                let image =
                    egui::Image::from_texture(texture).fit_to_exact_size(Vec2::splat(icon_size));
                ui.add(image).on_hover_text(path.to_string_lossy());

                ui.label(format!(
                    "{}",
                    path.extension()
                        .map(|e| e.to_string_lossy())
                        .unwrap_or("UNKNOWN".into())
                ));

                ui.label(name).on_hover_text(path.to_string_lossy());
            }
            Icon::Error { path, name, error } => {
                ui.colored_label(
                    egui::Color32::RED,
                    format!("❌\n{}\n{}", &name[..name.len().min(10)], error),
                )
                .on_hover_text(path.to_string_lossy());
            }
        }
    });
}

fn handle_key_events(ctx: &egui::Context) {
    // Handle keyboard shortcuts
    ctx.input(|i| {
        if i.key_pressed(egui::Key::Escape)
            || (i.modifiers.ctrl && i.key_pressed(egui::Key::D))
            || (i.modifiers.ctrl && i.key_pressed(egui::Key::C))
        {
            info!("User requested application close via keyboard shortcut");

            // TODO find out why this is causing the program to freeze/hang
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    });
}

impl eframe::App for IconViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set theme based on dark_mode toggle
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        handle_key_events(ctx);

        // set scaling for high-dpi display so the ui doesn't render too small
        ctx.set_pixels_per_point(2.0);

        // Check if icons are still loading and request repaint to keep UI responsive
        let current_icon_count = self.icons.len();
        
        // Toolbar
        self.render_top_bar(ctx);

        self.render_main_panel(ctx);
        
        // If we got new icons during this frame, request another repaint
        // This ensures the UI updates as soon as new icons are available
        if self.icons.len() > current_icon_count {
            ctx.request_repaint();
        } else {
            // Even if no new icons arrived, request a repaint after a short delay
            // to check for new icons periodically while loading is in progress
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
        }
    }
}
