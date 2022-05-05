use eframe::{run_native};
use egui::Vec2;

mod symlink_gui;

fn main() {
    let app = 
    symlink_gui::Settings::new(
        symlink_gui::Symlink::new());
    let icon = image::open("icon.png")
        .expect("Failed to open icon path")
        .to_rgba8();
    let size = &(icon.width(), icon.height());
    let options = eframe::NativeOptions {
        icon_data: Some(eframe::epi::IconData {
            rgba: icon.into_raw(),
            width: size.0,
            height: size.1,
        }),
        initial_window_size: Some(Vec2::new(560.0, 240.0)),
        ..Default::default()
    };

    run_native(Box::new(app), options);
}
