use crate::app_state::{EngineCommand, TitanAppState};
use eframe::egui;

/// Il pannello centrale per la chat.
pub fn show(ctx: &egui::Context, state: &mut TitanAppState) {
    // 1. PRIMA blocchiamo lo spazio in basso per la barra di testo (così non sparisce!)
    egui::TopBottomPanel::bottom("input_panel")
        .frame(egui::Frame::default().inner_margin(10.0))
        .show(ctx, |ui| {
            render_input_area(ctx, ui, state);
        });

    // 2. POI diamo tutto lo spazio rimanente alla cronologia della chat
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(egui::RichText::new("Titan Local AI - Chat").strong());
        ui.add_space(10.0);
        ui.separator();

        // Area Cronologia Messaggi
        render_chat_history(ui, state);
    });
}

fn render_chat_history(ui: &mut egui::Ui, state: &TitanAppState) {
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut state.output_text.as_str())
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .interactive(false) // Solo lettura
                    .lock_focus(false),
            );
        });
}

fn render_input_area(ctx: &egui::Context, ui: &mut egui::Ui, state: &mut TitanAppState) {
    ui.horizontal(|ui| {
        let input_field = ui.add_enabled(
            !state.is_generating,
            egui::TextEdit::singleline(&mut state.input_text)
                .hint_text("Scrivi un messaggio...")
                .desired_width(ui.available_width() - 80.0),
        );

        if (ui.button("Invia").clicked()
            || (input_field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
            && !state.input_text.is_empty()
        {
            state.is_generating = true;
            state.output_text.push_str("\n\nTU: ");
            state.output_text.push_str(&state.input_text);
            state.output_text.push_str("\nTITAN: ");

            let _ = state
                .tx_to_engine
                .send(EngineCommand::Generate(state.input_text.clone()));
            state.input_text.clear();
            ctx.request_repaint();
        }

        // Bottone STOP (durante inferenza)
        if state.is_generating {
            if ui.button("⏹ Stop").clicked() {
                let _ = state.tx_to_engine.send(EngineCommand::Stop);
                state.is_generating = false;
            }
        }
    });
}
