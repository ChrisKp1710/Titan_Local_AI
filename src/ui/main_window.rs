use eframe::egui;
use crate::app_state::{TitanAppState, EngineEvent};
use super::{model_sidebar, chat_panel};

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
                EngineEvent::Finished => {
                    self.state.is_generating = false;
                    ctx.request_repaint();
                }
                EngineEvent::Error(err) => {
                    self.state.output_text.push_str(&format!("\nERRORE: {}", err));
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
    }
}
