mod app_state;
mod ui;
mod engine;

use app_state::{EngineCommand, EngineEvent, TitanAppState};
use ui::main_window::TitanWindow;
use engine::hardware_detect;
use crossbeam_channel::{unbounded};
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // 1. Configurazione Logger
    tracing_subscriber::fmt::init();

    // 2. Rilevamento Hardware Bare-Metal
    let hw = hardware_detect::detect_hardware();

    // 3. Creazione dei canali asincroni
    let (tx_to_engine, rx_from_ui) = unbounded::<EngineCommand>();
    let (tx_to_ui, rx_from_engine) = unbounded::<EngineEvent>();

    // 3. AVVIO DEL WORKER THREAD (Engine Mock)
    thread::spawn(move || {
        while let Ok(command) = rx_from_ui.recv() {
            match command {
                EngineCommand::Generate(prompt) => {
                    tracing::info!("Engine: Ricevuto prompt '{}'", prompt);
                    
                    let tokens = vec!["Ciao,", "sono", "Titan", "AI.", "Il", "tuo", "carro", "armato", "di", "efficienza."];
                    
                    for token in tokens {
                        thread::sleep(Duration::from_millis(150));
                        if tx_to_ui.send(EngineEvent::NewToken(token.to_string() + " ")).is_err() {
                            break;
                        }
                    }
                    let _ = tx_to_ui.send(EngineEvent::Finished);
                }
                EngineCommand::Stop => {
                    tracing::warn!("Engine: Generazione interrotta");
                }
            }
        }
    });

    // 4. Inizializzazione della UI
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0]) // Leggermente più grande per accomodare la sidebar
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Titan Local AI",
        options,
        Box::new(move |cc| {
            // Estetica Spartan: Dark Mode Forzata
            cc.egui_ctx.set_visuals(eframe::egui::Visuals::dark());
            
            // Inizializzazione Window Modulare
            let state = TitanAppState::new(
                tx_to_engine, 
                rx_from_engine, 
                hw.total_ram_gb, 
                hw.ram_model.clone(),
                hw.vram_gb, 
                hw.gpu_name.clone(), 
                hw.cpu_name.clone(),
                hw.cpu_cores,
                hw.cpu_threads,
                hw.is_high_end
            );
            Box::new(TitanWindow::new(state))
        }),
    )
    .map_err(|e| anyhow::anyhow!("Errore eframe: {}", e))
}
