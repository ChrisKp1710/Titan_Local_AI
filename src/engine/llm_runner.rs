use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use std::path::Path;
use anyhow::Result;

pub struct LlamaRunner {
    pub model: LlamaModel,
}

impl LlamaRunner {
    /// Carica i pesi del modello in VRAM utilizzando la "Beast Mode" (Offload Totale).
    pub fn load(backend: &LlamaBackend, path: &Path) -> Result<Self> {
        // Configurazione Parametri Modello (Fase 2 - Step 3)
        let params = LlamaModelParams::default()
            .with_n_gpu_layers(999) // Forza ogni singolo layer sulla GPU (Beast Mode)
            .with_use_mmap(true);   // Usa mappatura memoria virtuale per velocità istantanea
        
        // Caricamento fisico dei pesi (questa operazione avviene nel Worker Thread)
        let model = LlamaModel::load_from_file(backend, path, &params)
            .map_err(|e| anyhow::anyhow!("Errore caricamento modello GGUF: {:?}", e))?;
            
        Ok(Self { model })
    }
}
