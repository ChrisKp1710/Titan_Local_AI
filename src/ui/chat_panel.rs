use crate::app_state::{EngineCommand, TitanAppState};
use eframe::egui;

/// Il pannello centrale per la chat.
pub fn show(ctx: &egui::Context, state: &mut TitanAppState) {
    // 1. PRIMA blocchiamo lo spazio in basso per la barra di testo
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

        // Area Cronologia Messaggi con Reasoning Parser
        render_chat_history(ui, state);
    });
}

/// Renderizza la cronologia della chat parsando dinamicamente i blocchi <think>
fn render_chat_history(ui: &mut egui::Ui, state: &TitanAppState) {
    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            let text = &state.output_text;
            let mut current_pos = 0;

            // Parser logico per segmenti di testo
            while current_pos < text.len() {
                if let Some(think_start) = text[current_pos..].find("<think>") {
                    let think_start_abs = current_pos + think_start;
                    
                    // 1. Renderizza il testo normale prima del pensiero
                    if think_start_abs > current_pos {
                        let normal_text = &text[current_pos..think_start_abs];
                        render_normal_text(ui, normal_text);
                    }

                    // 2. Cerca la fine del pensiero
                    let content_start = think_start_abs + 7; // "<think>".len()
                    if let Some(think_end) = text[content_start..].find("</think>") {
                        let think_end_abs = content_start + think_end;
                        let thought_content = &text[content_start..think_end_abs];
                        
                        render_thought_block(ui, thought_content, false);
                        current_pos = think_end_abs + 8; // "</think>".len()
                    } else {
                        // Pensiero ancora in streaming (tag di chiusura mancante)
                        let thought_content = &text[content_start..];
                        render_thought_block(ui, thought_content, true);
                        current_pos = text.len();
                    }
                } else {
                    // 3. Renderizza il resto del testo normale se non ci sono più tag <think>
                    let normal_text = &text[current_pos..];
                    render_normal_text(ui, normal_text);
                    current_pos = text.len();
                }
            }
        });
}

/// Helper per renderizzare testo normale con wrapping solido
fn render_normal_text(ui: &mut egui::Ui, text: &str) {
    if !text.is_empty() {
        ui.add(
            egui::Label::new(
                egui::RichText::new(text.trim_start_matches('\n'))
                    .monospace()
                    .size(14.0)
            ).wrap(true)
        );
        ui.add_space(5.0);
    }
}

/// Helper per renderizzare il blocco di pensiero collassabile
fn render_thought_block(ui: &mut egui::Ui, text: &str, is_streaming: bool) {
    egui::CollapsingHeader::new(egui::RichText::new("🧠 Processo di Ragionamento").strong())
        .default_open(is_streaming) // Aperto di default se sta ancora scrivendo
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(text)
                        .italics()
                        .color(egui::Color32::GRAY)
                ).wrap(true)
            );
        });
    ui.add_space(5.0);
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

            let _ = state.tx_to_engine.send(EngineCommand::Generate {
                prompt: state.input_text.clone(),
                temperature: state.temperature,
                max_tokens: state.max_tokens,
            });
            state.input_text.clear();
            ctx.request_repaint();
        }

        if state.is_generating {
            if ui.button("⏹ Stop").clicked() {
                let _ = state.tx_to_engine.send(EngineCommand::Stop);
                state.is_generating = false;
            }
        }
    });
}
