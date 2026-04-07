use crossbeam_channel::{Receiver, Sender};

/// Comandi inviati dalla UI verso l'Engine
pub enum EngineCommand {
    Generate {
        prompt: String,
        temperature: f32,
        max_tokens: u32,
    },
    LoadModel(std::path::PathBuf),
    Stop,
}

/// Eventi inviati dall'Engine verso la UI (Streaming)
pub enum EngineEvent {
    NewToken(String),
    ModelMetadataLoaded(String),
    ModelLoadedSuccess(String),
    Finished,
    #[allow(dead_code)]
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

    // Parametri di Inferenza (Fase 2 - Step 5)
    pub temperature: f32,
    pub max_tokens: u32,

    // Dati Hardware
    pub total_ram_gb: f32,
    pub ram_model: String,
    pub vram_gb: f32,
    pub gpu_name: String,
    pub cpu_name: String,
    pub cpu_cores: usize,
    pub cpu_threads: usize,
    pub is_high_end: bool,
    pub current_model: String,
}

impl TitanAppState {
    pub fn new(
        tx_to_engine: Sender<EngineCommand>, 
        rx_from_engine: Receiver<EngineEvent>,
        total_ram_gb: f32,
        ram_model: String,
        vram_gb: f32,
        gpu_name: String,
        cpu_name: String,
        cpu_cores: usize,
        cpu_threads: usize,
        is_high_end: bool,
    ) -> Self {
        Self {
            tx_to_engine,
            rx_from_engine,
            input_text: String::new(),
            output_text: String::new(),
            is_generating: false,
            // Default di fabbrica per Titan AI
            temperature: 0.7,
            max_tokens: 1024,
            total_ram_gb,
            ram_model,
            vram_gb,
            gpu_name,
            cpu_name,
            cpu_cores,
            cpu_threads,
            is_high_end,
            current_model: "Nessun modello caricato".to_string(),
        }
    }
}
