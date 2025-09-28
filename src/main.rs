use icon_viewer::icon_viewer_app::IconViewerApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "Program Icon Viewer",
        options,
        Box::new(|cc| Ok(Box::new(IconViewerApp::new(cc)))),
    )
}
