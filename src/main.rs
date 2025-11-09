use eframe::egui;  
use std::fs;       
use rfd::FileDialog;  
#[cfg(target_os = "linux")]
use std::path::Path;   
use std::process::Command; 

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])  
            .with_title("HIYC File Locker"),        
        ..Default::default()
    };

    
    eframe::run_native(
        "HIYC",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
struct MyApp {
    filename: String,      
    password: String,      
    message: String,      
    message_color: egui::Color32,  
    last_output: Option<String>,   
    show_password: bool,          
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            filename: String::new(),           
            password: String::new(),          
            message: String::new(),           
            message_color: egui::Color32::RED,
            last_output: None,  
            show_password: false,
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.heading(egui::RichText::new("HIYC File Locker")
                    .size(40.0)  
                    .strong());
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Encrypt and decrypt ANY file with a password")
                    .size(16.0)  
                    .color(egui::Color32::GRAY));
                
                ui.add_space(40.0);
            });
            
            ui.vertical_centered(|ui| {
                ui.set_max_width(600.0);
                ui.separator();
                ui.add_space(30.0);
                ui.group(|ui| {
                    ui.set_min_height(80.0);
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("Select File").size(18.0).strong());
                        ui.add_space(10.0);
                        ui.horizontal(|ui: &mut egui::Ui| {
                            let text_edit = egui::TextEdit::singleline(&mut self.filename)
                                .hint_text("Click Browse to select a file...")
                                .desired_width(ui.available_width() - 120.0);
                            ui.add(text_edit);
                            if ui.add_sized([100.0, 30.0], egui::Button::new("Browse")).clicked() {
                                if let Some(path) = FileDialog::new().pick_file() {
                                    self.filename = path.display().to_string();
                                }
                            }
                        });


                        if !self.filename.is_empty() {
                            ui.add_space(6.0);
                            if ui.button("Reveal in Folder").clicked() {
                                let _ = reveal_in_file_manager(&self.filename);
                            }
                        }
                    });
                });
                ui.add_space(20.0);
                ui.group(|ui| {
                    ui.set_min_height(80.0);
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("🔑 Password").size(18.0).strong());
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            let password_field = egui::TextEdit::singleline(&mut self.password)
                                .password(!self.show_password)
                                .hint_text("Enter your password")
                                .desired_width(ui.available_width() - 70.0);
                            ui.add(password_field);
                            
                            let show_hide_label = if self.show_password { "👁 Hide" } else { "👁 Show" };
                            if ui.add_sized([60.0, 28.0], egui::Button::new(show_hide_label)).clicked() {
                                self.show_password = !self.show_password;
                            }
                        });
                        
                        // Password strength indicator
                        let (strength_text, strength_color) = password_strength(&self.password);
                        if !strength_text.is_empty() {
                            ui.add_space(6.0);
                            ui.label(egui::RichText::new(strength_text).color(strength_color));
                        }
                    });
                });
                ui.add_space(30.0);
                ui.horizontal(|ui| {
                    ui.add_space((ui.available_width() - 320.0) / 2.0);  // Center buttons
                    
                    if ui.add_sized([150.0, 50.0], 
                        egui::Button::new(egui::RichText::new("🔒 Lock File").size(16.0))
                            .fill(egui::Color32::from_rgb(220, 80, 80)))
                        .clicked() {
                        self.lock_file();
                    }
                    
                    ui.add_space(20.0);
                    
                    if ui.add_sized([150.0, 50.0], 
                        egui::Button::new(egui::RichText::new("🔓 Unlock File").size(16.0))
                            .fill(egui::Color32::from_rgb(80, 180, 80)))
                        .clicked() {
                        self.unlock_file();
                    }
                });
                
                ui.add_space(30.0);
                ui.separator();
                ui.add_space(20.0);
                
                if !self.message.is_empty() {
                    ui.group(|ui| {
                        ui.set_min_height(60.0);
                        ui.vertical_centered(|ui| {
                            ui.add_space(10.0);
                            ui.label(egui::RichText::new(&self.message)
                                .size(16.0)
                                .color(self.message_color));
                            if self.message_color == egui::Color32::GREEN {
                                if let Some(path) = &self.last_output {
                                    ui.add_space(8.0);
                                    if ui.button("Reveal in Folder").clicked() {
                                        let _ = reveal_in_file_manager(path);
                                    }
                                }
                            }
                        });
                    });
                }
            });
        });
    }
}

impl MyApp {
    fn lock_file(&mut self) {
        if self.filename.is_empty() {
            self.show_error("Please enter the filename!");
            return;
        }
        if self.password.is_empty() {
            self.show_error("Please enter the password!");
            return;

        }
        
        match fs::read(&self.filename) {
            Ok(contents) => {
                // Encrypting the content in the file
                let encrypted = xor_encrypt_decrypt(&contents, &self.password);
                let locked_filename = format!("{}.locked", self.filename);
                match fs::write(&locked_filename, encrypted) {
                    Ok(_) => {
                        self.last_output = Some(locked_filename.clone());
                        self.show_success(&format!("File locked!\nSaved as: {}", locked_filename));
                    },
                    Err(e) => {
                        self.show_error(&format!("Error writing file: {}", e));
                    }
                }
            },
            Err(e) => {
                self.show_error(&format!("Error reading file: {}", e));
            }
        }
    }
    
    fn unlock_file(&mut self) {
        if self.filename.is_empty() {
            self.show_error("Enter file name!");
            return;
        }
        if self.password.is_empty() {
            self.show_error("Enter password!");
            return;
        }
        match fs::read(&self.filename) {
            Ok(contents) => {
                let decrypted = xor_encrypt_decrypt(&contents, &self.password);
                let unlocked_filename = if self.filename.ends_with(".locked") {
                    self.filename.trim_end_matches(".locked").to_string()
                } else {
                    self.filename.clone()
                };
                
                match fs::write(&unlocked_filename, decrypted) {
                    Ok(_) => {
                        self.last_output = Some(unlocked_filename.clone());
                        self.show_success(&format!("File unlocked!\nRestored as: {}", unlocked_filename));
                    },
                    Err(e) => {
                        self.show_error(&format!("Error writing file: {}", e));
                    }
                }
            },
            Err(e) => {
                self.show_error(&format!("Error reading file: {}", e));
            }
        }
    }
    

    fn show_success(&mut self, message: &str) {
        self.message = message.to_string();
        self.message_color = egui::Color32::GREEN;
    }
    fn show_error(&mut self, message: &str) {
        self.message = message.to_string();
        self.message_color = egui::Color32::RED;
    }
}

fn xor_encrypt_decrypt(data: &[u8], password: &str) -> Vec<u8> {
    let key = password.as_bytes();
    
    if key.is_empty() {
        return data.to_vec();
    }
    
    let mut result = Vec::new();
    
    for (i, &byte) in data.iter().enumerate() {
        let key_byte = key[i % key.len()];
        let encrypted_byte = byte ^ key_byte;
        result.push(encrypted_byte);
    }
    
    result
}

// Password strength calculator
fn password_strength(pw: &str) -> (&'static str, egui::Color32) {
    let len = pw.len();
    if len == 0 {
        ("", egui::Color32::TRANSPARENT)
    } else if len < 6 {
        ("Strength: Weak", egui::Color32::from_rgb(220, 100, 100))
    } else if len < 10 {
        ("Strength: Medium", egui::Color32::from_rgb(220, 180, 100))
    } else {
        let has_num = pw.chars().any(|c| c.is_ascii_digit());
        let has_sym = pw.chars().any(|c| !c.is_ascii_alphanumeric());
        if has_num && has_sym {
            ("Strength: Strong", egui::Color32::from_rgb(100, 200, 100))
        } else {
            ("Strength: Medium", egui::Color32::from_rgb(220, 180, 100))
        }
    }
}

// Open the folder in the file manager
fn reveal_in_file_manager(path: &str) -> std::io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("open").args(["-R", path]).status()?;
    }
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("explorer").arg("/select,").arg(path).status()?;
    }
    #[cfg(target_os = "linux")]
    {
        let dir = Path::new(path).parent().unwrap_or(Path::new("."));
        let _ = Command::new("xdg-open").arg(dir).status()?;
    }
    Ok(())
}
