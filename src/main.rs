mod app_state;
mod ui;
mod engine;
mod models; // Modulo per il parsing dell'header GGUF

use app_state::{EngineCommand, EngineEvent, TitanAppState};
use ui::main_window::TitanWindow;
use engine::{hardware_detect, llm_runner::LlamaRunner};
use llama_cpp_2::llama_backend::LlamaBackend;
use crossbeam_channel::{unbounded};
use std::thread;

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
        // Inizializzazione Unica del Backend (Fase 2 - Step 3)
        let backend = LlamaBackend::init().expect("ERRORE: Impossibile inizializzare llama.cpp backend");
        let mut loaded_runner: Option<LlamaRunner> = None;

        while let Ok(command) = rx_from_ui.recv() {
            match command {
                EngineCommand::Generate(prompt) => {
                    tracing::info!("Engine: Ricevuto prompt '{}'", prompt);
                    // Integrazione Reale (Fase 2 - Step 4)
                    if let Some(runner) = &loaded_runner {
                        if let Err(e) = runner.generate(&backend, &prompt, &tx_to_ui, &rx_from_ui) {
                            let _ = tx_to_ui.send(EngineEvent::Error(format!("Errore Generazione: {}", e)));
                        }
                    } else {
                        let _ = tx_to_ui.send(EngineEvent::Error("Nessun modello caricato!".to_string()));
                    }
                    let _ = tx_to_ui.send(EngineEvent::Finished);
                }
                EngineCommand::LoadModel(path) => {
                    // 1. Parsing metadati veloce (Zero-Memory)
                    match models::gguf_parser::parse_gguf_metadata(&path) {
                        Ok(meta) => {
                            let report = format!(
                                "Dati GGUF: v{} | Tensors: {} | KV: {}",
                                meta.version, meta.tensor_count, meta.metadata_kv_count
                            );
                            let _ = tx_to_ui.send(EngineEvent::ModelMetadataLoaded(report));

                            // 2. Caricamento fisico dei PESI in VRAM (Beast Mode)
                            tracing::info!("Engine: Caricamento pesi GGUF in VRAM...");
                            match LlamaRunner::load(&backend, &path) {
                                Ok(runner) => {
                                    loaded_runner = Some(runner);
                                    let _ = tx_to_ui.send(EngineEvent::ModelLoadedSuccess("Pesi caricati in VRAM. Pronto per l'inferenza.".to_string()));
                                }
                                Err(e) => {
                                    let _ = tx_to_ui.send(EngineEvent::Error(format!("Errore VRAM Load: {}", e)));
                                }
                            }
                        }
                        Err(e) => {
                            let _ = tx_to_ui.send(EngineEvent::Error(format!("Errore Header: {}", e)));
                        }
                    }
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
