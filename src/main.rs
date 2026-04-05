mod app_state;
use app_state::{EngineCommand, EngineEvent, TitanAppState};
use eframe::egui;
use crossbeam_channel::{unbounded};
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // Configurazione Logger
    tracing_subscriber::fmt::init();

    // Creazione dei canali asincroni
    let (tx_to_engine, rx_from_ui) = unbounded::<EngineCommand>();
    let (tx_to_ui, rx_from_engine) = unbounded::<EngineEvent>();

    // AVVIO DEL WORKER THREAD (Engine Mock)
    // Questo thread simula l'inferenza senza bloccare la UI
    thread::spawn(move || {
        while let Ok(command) = rx_from_ui.recv() {
            match command {
                EngineCommand::Generate(prompt) => {
                    tracing::info!("Engine: Ricevuto prompt '{}'", prompt);
                    
                    // Simulazione di generazione token a intervallo costante
                    let tokens = vec!["Ciao,", "sono", "Titan", "AI.", "Il", "tuo", "carro", "armato", "di", "efficienza."];
                    
                    for token in tokens {
                        thread::sleep(Duration::from_millis(150)); // Simula latenza inferenza
                        let _ = tx_to_ui.send(EngineEvent::NewToken(token.to_string() + " "));
                    }
                    let _ = tx_to_ui.send(EngineEvent::Finished);
                }
                EngineCommand::Stop => {
                    tracing::warn!("Engine: Generazione interrotta");
                }
            }
        }
    });

    // Inizializzazione della finestra eframe
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Titan Local AI",
        options,
        Box::new(|cc| {
            // Forziamo il Visuals::dark() per rispettare ROADMAP_AND_UX.md
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(TitanApp::new(TitanAppState::new(tx_to_engine, rx_from_engine)))
        }),
    )
    .map_err(|e| anyhow::anyhow!("Errore eframe: {}", e))
}

struct TitanApp {
    state: TitanAppState,
}

impl TitanApp {
    fn new(state: TitanAppState) -> Self {
        Self { state }
    }
}

impl eframe::App for TitanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // RICEZIONE ASINCRONA DEGLI EVENTI
        // Usiamo try_recv() per non bloccare il Main Thread (UI a 60fps)
        while let Ok(event) = self.state.rx_from_engine.try_recv() {
            match event {
                EngineEvent::NewToken(token) => {
                    self.state.output_text.push_str(&token);
                    // Forza il refresh immediato alla ricezione di ogni token
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

        // LAYOUT UI (Spartan & Pure)
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Titan Local AI - Fase 1 Skeleton");
            ui.add_space(10.0);

            // Area Output (Chat History finta)
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.state.output_text)
                            .font(egui::TextStyle::Monospace) // Look da terminale
                            .desired_width(f32::INFINITY)
                            .lock_focus(true)
                    );
                });

            ui.add_space(10.0);
            ui.separator();

            // Area Input
            ui.horizontal(|ui| {
                let input_field = ui.add_enabled(
                    !self.state.is_generating,
                    egui::TextEdit::singleline(&mut self.state.input_text)
                        .hint_text("Scrivi un messaggio...")
                );

                if (ui.button("Invia").clicked() || (input_field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
                    && !self.state.input_text.is_empty() 
                {
                    self.state.is_generating = true;
                    self.state.output_text.push_str("\n\nTU: ");
                    self.state.output_text.push_str(&self.state.input_text);
                    self.state.output_text.push_str("\nTITAN: ");
                    
                    let _ = self.state.tx_to_engine.send(EngineCommand::Generate(self.state.input_text.clone()));
                    self.state.input_text.clear();
                    
                    // Richiediamo il repaint per mostrare subito il messaggio dell'utente
                    ctx.request_repaint();
                }
            });
        });
    }
}
