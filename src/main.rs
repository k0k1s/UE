use eframe::egui;
use eframe::egui::RichText;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Select Menu Example",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    ).unwrap();
}

struct MyApp {
    selected_item: usize,
    items: Vec<&'static str>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            selected_item: 0,
            items: vec!["Option 1", "Option 2", "Option 3"],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // Center the header
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Select Menu Example")
                            .heading()
                            .size(24.0)
                    );
                });

                ui.add_space(20.0);

                // Center the combo box
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label("Select an option")
                        .selected_text(self.items[self.selected_item])
                        .show_ui(ui, |ui| {
                            for (index, item) in self.items.iter().enumerate() {
                                ui.selectable_value(&mut self.selected_item, index, *item);
                            }
                        });
                });

                ui.add_space(20.0);

                // Center the selected item display
                ui.horizontal(|ui| {
                    ui.label(format!("You selected: {}", self.items[self.selected_item]));
                });

                ui.add_space(40.0);

                // Center the footer
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                        ui.label("MyApp by Your Name");
                        ui.hyperlink("https://github.com/your-username/your-repo");
                    });
                });
            });
        });
    }
}
