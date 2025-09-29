use anyhow::Result;
use eframe::egui;
use egui::{ColorImage, TextureHandle, Vec2};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info, trace, warn};
use walkdir::WalkDir;

/// Maximum depth to recurse below "icons" or "pixmaps" paths
const MAX_ICON_SEARCH_DEPTH: usize = 4;

struct IconInfo {
    path: PathBuf,
    name: String,
    texture: Option<TextureHandle>,
    load_error: Option<String>,
}

pub struct IconViewerApp {
    icons: Vec<IconInfo>,
    scroll_area_id: egui::Id,
    dark_mode: bool,
    icon_size: f32,
    icon_size_options: Vec<(String, f32)>,
}

impl IconViewerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        info!("Initializing IconViewerApp");
        let mut app = Self {
            icons: Vec::new(),
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

        app.discover_icons();
        app.load_icon_textures(&cc.egui_ctx);

        info!(
            "IconViewerApp initialization complete with {} icons",
            app.icons.len()
        );
        app
    }

    fn discover_icons(&mut self) {
        info!("Starting icon discovery");
        let icon_paths = Self::get_icon_search_paths();
        let mut found_icons = HashMap::new();

        for search_path in icon_paths {
            if !search_path.exists() {
                warn!("Skipping non-existent path: {}", search_path.display());
                continue;
            }

            debug!("Searching for icons in: {}", search_path.display());

            for entry in WalkDir::new(&search_path)
                .max_depth(MAX_ICON_SEARCH_DEPTH)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if matches!(
                        ext.as_str(),
                        "png" | "svg" | "ico" | "xpm" | "jpg" | "jpeg" | "gif" | "bmp"
                    ) {
                        if let Some(name) = path.file_stem() {
                            let name_str = name.to_string_lossy().to_string();
                            // Avoid duplicates, prefer higher resolution or more common formats
                            if !found_icons.contains_key(&name_str)
                                || matches!(ext.as_str(), "png" | "svg")
                            {
                                trace!("Found icon: {} at {}", name_str, path.display());
                                found_icons.insert(name_str.clone(), path.to_path_buf());
                            }
                        }
                    }
                }
            }
        }

        self.icons = found_icons
            .into_iter()
            .map(|(name, path)| IconInfo {
                path,
                name,
                texture: None,
                load_error: None,
            })
            .collect();

        // Sort by name for consistent display
        self.icons.sort_by(|a, b| a.name.cmp(&b.name));

        info!(
            "Icon discovery complete. Found {} unique icons",
            self.icons.len()
        );
    }

    fn get_icon_search_paths() -> Vec<PathBuf> {
        let mut paths = Vec::new();

        // Paths to search for icons:
        //       /run/current-system/sw/share/icons
        //       /etc/profiles/per-user/egrieco/share/icons
        //       /home/egrieco/.nix-profile/share/icons

        // NixOS-specific paths
        paths.push(PathBuf::from("/run/current-system/sw/share/icons"));
        paths.push(PathBuf::from("/run/current-system/sw/share/pixmaps"));

        // User profile paths for Nix
        if let Ok(user) = std::env::var("USER") {
            paths.push(PathBuf::from(format!(
                "/etc/profiles/per-user/{user}/share/icons"
            )));
            paths.push(PathBuf::from(format!(
                "/etc/profiles/per-user/{user}/share/pixmaps"
            )));
        }
        if let Ok(home) = std::env::var("HOME") {
            paths.push(PathBuf::from(format!("{}/.nix-profile/share/icons", home)));
            paths.push(PathBuf::from(format!(
                "{}/.nix-profile/share/pixmaps",
                home
            )));
        }

        // Common Linux icon directories (still useful for some packages)
        paths.push(PathBuf::from("/usr/share/icons"));
        paths.push(PathBuf::from("/usr/share/pixmaps"));
        paths.push(PathBuf::from("/usr/local/share/icons"));
        paths.push(PathBuf::from("/usr/local/share/pixmaps"));

        // User-specific directories
        if let Ok(home) = std::env::var("HOME") {
            paths.push(PathBuf::from(format!("{}/.local/share/icons", home)));
            paths.push(PathBuf::from(format!("{}/.icons", home)));
        }

        // Flatpak icons
        paths.push(PathBuf::from("/var/lib/flatpak/exports/share/icons"));
        if let Ok(home) = std::env::var("HOME") {
            paths.push(PathBuf::from(format!(
                "{}/.local/share/flatpak/exports/share/icons",
                home
            )));
        }

        // Additional paths to search
        // /nix/store/qg05qi8y7y6jk6ys5fnx6xx4qns2ldhy-ghostty-1.1.3/share/icons
        // /nix/store/rpijzq6c40bdgy0ij7hsz9b2z3a8kbfn-gnome-shell-48.4/share/icons

        paths
    }

    fn load_icon_textures(&mut self, ctx: &egui::Context) {
        info!("Loading icon textures for {} icons", self.icons.len());
        let mut loaded_count = 0;
        let mut error_count = 0;

        for icon in &mut self.icons {
            match Self::load_icon_image(&icon.path) {
                Ok(color_image) => {
                    let texture =
                        ctx.load_texture(&icon.name, color_image, egui::TextureOptions::default());
                    icon.texture = Some(texture);
                    loaded_count += 1;
                    trace!("Successfully loaded texture for: {}", icon.name);
                }
                Err(e) => {
                    error!("Failed to load icon {}: {}", icon.name, e);
                    icon.load_error = Some(format!("Failed to load: {}", e));
                    error_count += 1;
                }
            }
        }

        info!(
            "Texture loading complete: {} loaded, {} errors",
            loaded_count, error_count
        );
    }

    fn load_icon_image(path: &Path) -> Result<ColorImage> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        debug!("Loading image: {} (type: {})", path.display(), extension);

        match extension.as_str() {
            "svg" => Self::load_svg_image(path),
            "xpm" => Self::load_xpm_image(path),
            _ => Self::load_raster_image(path),
        }
    }

    fn load_svg_image(path: &Path) -> Result<ColorImage> {
        trace!("Loading SVG image: {}", path.display());
        let svg_data = std::fs::read_to_string(path)?;
        debug!("Parsing SVG tree...");
        let usvg_tree = usvg::Tree::from_str(&svg_data, &usvg::Options::default())?;

        let size = usvg_tree.size();
        let width = size.width() as u32;
        let height = size.height() as u32;

        // Limit size to prevent memory issues
        let (width, height) = if width > 256 || height > 256 {
            let scale = 256.0 / width.max(height) as f32;
            debug!(
                "Scaling SVG from {}x{} to {}x{}",
                size.width() as u32,
                size.height() as u32,
                (width as f32 * scale) as u32,
                (height as f32 * scale) as u32
            );
            (
                (width as f32 * scale) as u32,
                (height as f32 * scale) as u32,
            )
        } else {
            (width, height)
        };

        let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height)
            .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap"))?;

        resvg::render(
            &usvg_tree,
            resvg::tiny_skia::Transform::from_scale(
                width as f32 / size.width(),
                height as f32 / size.height(),
            ),
            &mut pixmap.as_mut(),
        );

        let pixels = pixmap.data();
        let mut rgba_pixels = Vec::with_capacity(pixels.len());

        // Convert BGRA to RGBA
        for chunk in pixels.chunks_exact(4) {
            rgba_pixels.push(chunk[2]); // R
            rgba_pixels.push(chunk[1]); // G
            rgba_pixels.push(chunk[0]); // B
            rgba_pixels.push(chunk[3]); // A
        }

        Ok(ColorImage::from_rgba_unmultiplied(
            [width as usize, height as usize],
            &rgba_pixels,
        ))
    }

    fn load_raster_image(path: &Path) -> Result<ColorImage> {
        trace!("Loading raster image: {}", path.display());
        let img = image::open(path)?;

        // Limit size to prevent memory issues
        let img = if img.width() > 256 || img.height() > 256 {
            debug!(
                "Resizing raster image from {}x{} to max 256x256",
                img.width(),
                img.height()
            );
            img.resize(256, 256, image::imageops::FilterType::Lanczos3)
        } else {
            img
        };

        let img = img.to_rgba8();
        let (width, height) = img.dimensions();

        Ok(ColorImage::from_rgba_unmultiplied(
            [width as usize, height as usize],
            img.as_raw(),
        ))
    }

    fn load_xpm_image(path: &Path) -> Result<ColorImage> {
        // For XPM files, we'll create a placeholder icon since the image crate doesn't support XPM
        // In a full implementation, you'd want to use a dedicated XPM parser
        warn!(
            "XPM format not fully supported, creating placeholder for: {}",
            path.display()
        );
        let content = std::fs::read_to_string(path)?;

        // Try to extract dimensions from XMP header if possible
        let (width, height) =
            if let Some(width_line) = content.lines().find(|line| line.contains("width")) {
                // Simple parsing - this is a basic implementation
                let width = width_line
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap_or(32);
                let height = content
                    .lines()
                    .find(|line| line.contains("height"))
                    .and_then(|line| {
                        line.chars()
                            .filter(|c| c.is_ascii_digit())
                            .collect::<String>()
                            .parse::<u32>()
                            .ok()
                    })
                    .unwrap_or(32);
                (width.min(64), height.min(64)) // Limit size
            } else {
                (32, 32) // Default size
            };

        // Create a simple placeholder pattern for XMP files
        let mut pixels = Vec::with_capacity((width * height * 4) as usize);
        for y in 0..height {
            for x in 0..width {
                // Create a simple checkerboard pattern to indicate it's an XMP file
                let is_dark = (x / 4 + y / 4) % 2 == 0;
                if is_dark {
                    pixels.extend_from_slice(&[100, 100, 100, 255]); // Dark gray
                } else {
                    pixels.extend_from_slice(&[200, 200, 200, 255]); // Light gray
                }
            }
        }

        Ok(ColorImage::from_rgba_unmultiplied(
            [width as usize, height as usize],
            &pixels,
        ))
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

fn render_icon(icon_size: f32, spacing: f32, ui: &mut egui::Ui, icon: &IconInfo) {
    // TODO is it possible to set hover text on a vertical?
    ui.vertical(|ui| {
        ui.set_width(icon_size + spacing);

        if let Some(texture) = &icon.texture {
            let image =
                egui::Image::from_texture(texture).fit_to_exact_size(Vec2::splat(icon_size));
            ui.add(image).on_hover_text(icon.path.to_string_lossy());
        } else if let Some(error) = &icon.load_error {
            ui.colored_label(
                egui::Color32::RED,
                format!("❌\n{}\n{}", &icon.name[..icon.name.len().min(10)], error),
            )
            .on_hover_text(icon.path.to_string_lossy());
        } else {
            ui.colored_label(egui::Color32::GRAY, "⏳")
                .on_hover_text(icon.path.to_string_lossy());
        }

        ui.label(format!(
            "{}",
            icon.path
                .extension()
                .map(|e| e.to_string_lossy())
                .unwrap_or("UNKNOWN".into())
        ));

        ui.label(&icon.name)
            .on_hover_text(icon.path.to_string_lossy());
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

        // Toolbar
        self.render_top_bar(ctx);

        self.render_main_panel(ctx);
    }
}
