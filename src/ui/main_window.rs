use super::{chat_panel, model_sidebar};
use crate::app_state::{EngineEvent, TitanAppState};
use eframe::egui;

/// La finestra principale dell'applicazione.
pub struct TitanWindow {
    pub state: TitanAppState,
}

impl TitanWindow {
    pub fn new(state: TitanAppState) -> Self {
        Self { state }
    }

    /// Aggiornamento asincrono dei canali di comunicazione.
    pub fn handle_events(&mut self, ctx: &egui::Context) {
        while let Ok(event) = self.state.rx_from_engine.try_recv() {
            match event {
                EngineEvent::NewToken(token) => {
                    self.state.output_text.push_str(&token);
                    ctx.request_repaint();
                }
                EngineEvent::ModelMetadataLoaded(report) => {
                    self.state
                        .output_text
                        .push_str(&format!("\n[SYSTEM] {}\n", report));
                    self.state.current_model = "Modello Attivo".to_string(); // Placeholder per ora
                    ctx.request_repaint();
                }
                EngineEvent::ModelLoadedSuccess(msg) => {
                    self.state
                        .output_text
                        .push_str(&format!("[SYSTEM] {}\n", msg));
                    self.state.is_generating = false;
                    ctx.request_repaint();
                }
                EngineEvent::Finished => {
                    self.state.is_generating = false;
                    ctx.request_repaint();
                }
                EngineEvent::ModelUnloaded => {
                    self.state
                        .output_text
                        .push_str("\n[SYSTEM] Modello scaricato correttamente. VRAM liberata.\n");
                    self.state.current_model = "Nessun modello caricato".to_string();
                    ctx.request_repaint();
                }
                EngineEvent::Error(err) => {
                    self.state
                        .output_text
                        .push_str(&format!("\n[ERROR] {}", err));
                    self.state.is_generating = false;
                }
            }
        }
    }
}

impl eframe::App for TitanWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Gestione eventi dal Worker Thread
        self.handle_events(ctx);

        // Layout Modulare
        model_sidebar::show(ctx, &mut self.state);
        chat_panel::show(ctx, &mut self.state);

        // IL CAFFÈ PER LA UI: Forza l'aggiornamento continuo durante l'inferenza
        if self.state.is_generating {
            ctx.request_repaint();
        }
    }
}
