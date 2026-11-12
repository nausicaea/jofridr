use egui::ViewportBuilder;
use jofridr::{error::Error, frontend::terminal::Terminal};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_fullscreen(true)
            .with_drag_and_drop(true),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    let term = jofridr::backend::terminal::Terminal::default();

    eframe::run_native(
        "Mimisbrunnr – Terminal",
        options,
        Box::new(|_cc| Ok(Box::new(Terminal::new(term)))),
    )
    .map_err(|e| Error::Eframe(format!("{e}")))?;

    Ok(())
}
