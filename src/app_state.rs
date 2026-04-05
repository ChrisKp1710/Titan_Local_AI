use eframe::egui;
use crossbeam_channel::{Receiver, Sender};

/// Comandi inviati dalla UI verso l'Engine
pub enum EngineCommand {
    Generate(String),
    Stop,
}

/// Eventi inviati dall'Engine verso la UI (Streaming)
pub enum EngineEvent {
    NewToken(String),
    Finished,
    Error(String),
}

/// Lo stato globale dell'applicazione Titan AI
pub struct TitanAppState {
    // Canali di comunicazione
    pub tx_to_engine: Sender<EngineCommand>,
    pub rx_from_engine: Receiver<EngineEvent>,

    // Stato locale della UI
    pub input_text: String,
    pub output_text: String,
    pub is_generating: bool,
}

impl TitanAppState {
    pub fn new(tx_to_engine: Sender<EngineCommand>, rx_from_engine: Receiver<EngineEvent>) -> Self {
        Self {
            tx_to_engine,
            rx_from_engine,
            input_text: String::new(),
            output_text: String::new(),
            is_generating: false,
        }
    }
}
