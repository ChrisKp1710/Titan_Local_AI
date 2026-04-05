use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::sampling::LlamaSampler;
use llama_cpp_2::llama_backend::LlamaBackend;
use crate::app_state::{EngineEvent, EngineCommand};
use crossbeam_channel::{Sender, Receiver};
use std::path::Path;
use anyhow::{Result, anyhow};

pub struct LlamaRunner {
    pub model: LlamaModel,
}

impl LlamaRunner {
    /// Carica i pesi del modello in VRAM con offloading totale (GPU AMD Radeon RX 7900 XTX).
    pub fn load(backend: &LlamaBackend, path: &Path) -> Result<Self> {
        let params = LlamaModelParams::default()
            .with_n_gpu_layers(999) 
            .with_use_mmap(true);
        
        let model = LlamaModel::load_from_file(backend, path, &params)
            .map_err(|e| anyhow!("Errore caricamento modello GGUF: {:?}", e))?;
            
        Ok(Self { model })
    }

    /// Genera testo in tempo reale con streaming a latenza zero.
    pub fn generate(&self, backend: &LlamaBackend, prompt: &str, tx: &Sender<EngineEvent>, rx: &Receiver<EngineCommand>) -> Result<()> {
        // 1. Inizializzazione Contesto (n_ctx = 4096 tokens per Beast Mode)
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(std::num::NonZeroU32::new(4096));
        
        let mut ctx = self.model.new_context(backend, ctx_params)
            .map_err(|e| anyhow!("Errore creazione contesto: {:?}", e))?;

        // 2. Tokenizzazione del Prompt
        let formatted_prompt = format!("\nUser: {}\nTitan: ", prompt);
        let tokens = self.model.str_to_token(&formatted_prompt, llama_cpp_2::model::AddBos::Always)
            .map_err(|e| anyhow!("Errore tokenizzazione: {:?}", e))?;

        // 3. Preparazione inferenza iniziale (Batching prompt)
        let mut batch = LlamaBatch::new(512, 1);
        for (i, &token) in tokens.iter().enumerate() {
            batch.add(token, i as i32, &[0], i == tokens.len() - 1);
        }

        // 4. Configurazione Sampler (Temp 0.8 / Top-K 40 / Top-P 0.95 via Chain)
        let mut sampler = LlamaSampler::chain_simple([
            LlamaSampler::temp(0.8),
        ]);

        let mut n_cur = tokens.len() as i32;
        ctx.decode(&mut batch).map_err(|e| anyhow!("Errore decoding prompt: {:?}", e))?;

        loop {
            // Verifica interruzione utente (STOP Command)
            if let Ok(EngineCommand::Stop) = rx.try_recv() {
                break;
            }

            // 5. Campionamento (Firma 0.1.141 - richiede contesto e indice token nel batch)
            let idx = batch.n_tokens() - 1;
            let token_id = sampler.sample(&ctx, idx);
            
            // Controllo fine generazione
            if self.model.is_eog_token(token_id) {
                break;
            }

            // 6. Conversione Real-time (Token -> String via Vec<u8>)
            // Usiamo token_to_bytes per compatibilità stabile con 0.1.141
            let bytes = self.model.token_to_bytes(token_id, llama_cpp_2::model::Special::Tokenize)
                .map_err(|e| anyhow!("Errore conversione bytes: {:?}", e))?;
            
            let token_str = String::from_utf8_lossy(&bytes).to_string();
            
            // Streaming immediato alla UI per effetto "scrittura viva"
            let _ = tx.send(EngineEvent::NewToken(token_str));

            // 7. Preparazione batch per il token successivo
            batch.clear();
            batch.add(token_id, n_cur, &[0], true);
            n_cur += 1;

            ctx.decode(&mut batch).map_err(|e| anyhow!("Errore decoding loop: {:?}", e))?;
        }

        Ok(())
    }
}
