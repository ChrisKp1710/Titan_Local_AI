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
                render_resource_indicator(ui, state);
                ui.separator();
            });
        });
}

/// Indicatore di risorse e stato "Beast Mode"
fn render_resource_indicator(ui: &mut egui::Ui, state: &TitanAppState) {
    ui.vertical_centered(|ui| {
        ui.label(egui::RichText::new("HARDWARE DETECTED").strong().size(10.0));
        
        ui.add_space(5.0);

        // Visualizzazione RAM e VRAM Formattata
        ui.label(egui::RichText::new(format!(" {:.0} GB RAM", state.total_ram_gb)).strong());
        
        if !state.gpu_name.is_empty() && state.gpu_name != "Generic GPU" {
            ui.label(egui::RichText::new(&state.gpu_name).small().color(egui::Color32::GRAY));
            ui.label(egui::RichText::new(format!("󰢮 {:.0} GB VRAM", state.vram_gb)).strong());
        }

        ui.add_space(8.0);
        
        // Logica Dinamica: Mostra solo lo stato attivo
        if state.is_high_end {
            ui.label(egui::RichText::new("🚀 UNLEASHED / BEAST MODE")
                .color(egui::Color32::from_rgb(255, 165, 0))
                .strong());
        } else {
            ui.label(egui::RichText::new("🧊 Eco-Mode (Standard)")
                .color(egui::Color32::from_rgb(0, 150, 255))
                .small());
        }
    });
}
