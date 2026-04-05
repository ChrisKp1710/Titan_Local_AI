use eframe::egui;
use crate::app_state::TitanAppState;

/// La barra laterale sinistra per la gestione dei modelli e delle risorse.
pub fn show(ctx: &egui::Context, state: &mut TitanAppState) {
    egui::SidePanel::left("model_sidebar")
        .resizable(false)
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.add_space(10.0);
            ui.heading("💎 Modelli");
            
            ui.add_space(15.0);
            
            // Bottone Caricamento
            if ui.button("📁 Carica GGUF").clicked() {
                // Sarà implementato nella Fase 2
            }
            
            ui.add_space(20.0);
            ui.separator();
            
            // Lista modelli (Placeholder)
            ui.label("Modelli Locali:");
            ui.add_space(5.0);
            ui.selectable_label(false, "• Mistral-7B-v0.3.gguf");
            ui.selectable_label(false, "• Llama-3-8B-Q4.gguf");
            
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(10.0);
                render_resource_indicator(ui);
                ui.separator();
            });
        });
}

/// Indicatore di risorse e stato "Beast Mode"
fn render_resource_indicator(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new("RISORSE SISTEMA").strong().size(10.0));
        
        // Simula indicatore VRAM/RAM
        ui.add(egui::ProgressBar::new(0.65).text("RAM: 5.2GB / 8GB").desired_width(180.0));
        
        ui.add_space(5.0);
        
        // Placeholder Beast Mode (si attiva se hardware > 32GB)
        let beast_mode = false; // Mock
        if beast_mode {
            ui.label(egui::RichText::new("🔥 BEAST MODE ACTIVE")
                .color(egui::Color32::from_rgb(255, 100, 0))
                .strong());
        } else {
            ui.label(egui::RichText::new("🧊 Eco-Mode (Standard)")
                .color(egui::Color32::from_rgb(0, 150, 255))
                .small());
        }
    });
}
