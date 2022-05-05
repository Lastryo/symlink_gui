use std::path::PathBuf;

use eframe::{
    egui::Context,
    epi::{App, Frame},
};
use egui::CentralPanel;
use im_native_dialog::ImNativeFileDialog;

#[derive()]
pub struct Settings {
    symlink_settings: Symlink,
}

impl Settings {
    pub fn new(symlink_settings: Symlink) -> Settings {
        Settings {
            symlink_settings: symlink_settings
        }
    }
}

impl App for Settings {
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &Context, _frame: &Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if let Some(result) = self.symlink_settings.destination_dialog.check() {
                match result {
                    Ok(Some(path)) => self.symlink_settings.destination_directory = path,
                    Ok(None) => {}
                    Err(err) => {
                        eprintln!("error selecting xplane_path: {}", err)
                    }
                }
            }

            let destination_directory_path = &self
                .symlink_settings
                .destination_directory
                .to_string_lossy()
                .to_string();

            ui.horizontal(|ui| {
                ui.label("Destination directory: ");
                ui.horizontal(|ui| {
                    ui.label(destination_directory_path);
                    if ui.button("Browse").clicked() {
                        let location = self
                            .symlink_settings
                            .destination_directory
                            .parent()
                            .map(|location| location.to_path_buf());

                        let repaint_signal = _frame.request_repaint();
                        self.symlink_settings
                            .destination_dialog
                            .with_callback(move |_| repaint_signal)
                            .open_single_dir(location)
                            .expect("Unable to open file_path dialog");
                    }
                });
            });

            for n in 0..self.symlink_settings.directory_data.len() {
                if let Some(result) = self.symlink_settings.directory_data[n].dialog.check() {
                    match result {
                        Ok(Some(path)) => self.symlink_settings.directory_data[n].directory = path,
                        Ok(None) => {}
                        Err(err) => {
                            eprintln!("error selecting xplane_path: {}", err)
                        }
                    }
                }

                ui.horizontal(|ui| {
                    ui.label("folder: ");
                    ui.horizontal(|ui| {
                        ui.label(
                            self.symlink_settings.directory_data[n]
                                .directory
                                .to_string_lossy()
                                .to_string(),
                        );
                        if ui.button("Browse").clicked() {
                            let location = self.symlink_settings.directory_data[n]
                                .directory
                                .parent()
                                .map(|location| location.to_path_buf());

                            let repaint_signal = _frame.request_repaint();
                            self.symlink_settings.directory_data[n]
                                .dialog
                                .with_callback(move |_| repaint_signal)
                                .open_single_dir(location)
                                .expect("Unable to open file_path dialog");
                        }
                    });
                });
            }

            if ui.button("Add directory link").clicked() {
                self.symlink_settings
                    .directory_data
                    .push(DirectoryData::new());
            }

            if ui.button("Remove directory link").clicked() {
                self.symlink_settings.directory_data.pop();
            }

            if ui.button("Link").clicked() {
                for n in 0..self.symlink_settings.directory_data.len() {
                    let directory_path = self.symlink_settings.directory_data[n]
                        .directory
                        .to_string_lossy()
                        .to_string();
                    let split = directory_path.split("\\");
                    let directory_name = split.last().unwrap();
                    let destination = format!("{}\\{}", destination_directory_path, directory_name);
                    link_process(&destination, &directory_path);
                }
            }
        });
    }

    fn name(&self) -> &str {
        "symlink_gui"
    }
}

fn link_process(destination: &String, source: &String) {
    std::process::Command::new("cmd")
        .arg("/noprofile")
        .arg("/C")
        .arg("mklink")
        .arg("/D")
        .arg(destination)
        .arg(source)
        .spawn()
        .expect("sh command failed to start");
}

#[derive()]
pub struct Symlink {
    destination_dialog: ImNativeFileDialog<Option<PathBuf>>,
    destination_directory: PathBuf,
    directory_data: Vec<DirectoryData>,
}

impl Symlink {
    pub fn new() -> Symlink {
        Symlink {
            destination_dialog: Default::default(),
            destination_directory: Default::default(),

            directory_data: vec![],
        }
    }
}

#[derive()]
pub struct DirectoryData {
    dialog: ImNativeFileDialog<Option<PathBuf>>,
    directory: PathBuf,
}

impl DirectoryData {
    pub fn new() -> DirectoryData {
        DirectoryData {
            dialog: Default::default(),
            directory: Default::default(),
        }
    }
}

impl Default for DirectoryData {
    fn default() -> Self {
        Self::new()
    }
}
