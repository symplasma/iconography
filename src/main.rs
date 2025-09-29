use icon_viewer::icon_viewer_app::IconViewerApp;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn setup_tracing() -> Result<(), Box<dyn std::error::Error>> {
    // Get XDG cache directory for log files
    let cache_dir = dirs::cache_dir()
        .ok_or("Could not determine cache directory")?
        .join("icon-viewer");
    
    // Create cache directory if it doesn't exist
    std::fs::create_dir_all(&cache_dir)?;
    
    // Set up file appender
    let file_appender = tracing_appender::rolling::daily(cache_dir, "icon-viewer.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    // Set up tracing subscriber with both console and file output
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "icon_viewer=debug,info".into())
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stderr)
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
        )
        .init();
    
    // Keep the guard alive for the duration of the program
    std::mem::forget(_guard);
    
    Ok(())
}

fn main() -> Result<(), eframe::Error> {
    // Initialize tracing
    if let Err(e) = setup_tracing() {
        eprintln!("Failed to setup tracing: {}", e);
    }
    
    tracing::info!("Starting Icon Viewer application");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_maximized(true),
        ..Default::default()
    };

    let result = eframe::run_native(
        "Program Icon Viewer",
        options,
        Box::new(|cc| Ok(Box::new(IconViewerApp::new(cc)))),
    );
    
    tracing::info!("Icon Viewer application shutting down");
    result
}
