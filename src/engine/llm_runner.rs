use crate::app_state::{EngineEvent, EngineCommand};
use crossbeam_channel::{Sender, Receiver};
use std::path::Path;
use std::process::{Command, Child};
use std::os::windows::process::CommandExt;
use std::time::Duration;
use std::io::{BufRead, BufReader};
use anyhow::{Result, anyhow};
use serde_json::json;

pub struct LlamaRunner {
    pub child: Child,
    pub client: reqwest::blocking::Client,
}

impl LlamaRunner {
    /// Avvia il server llama-server.exe in modalità invisibile e carica il modello.
    pub fn load(path: &Path) -> Result<Self> {
        // Flag per Windows: CREATE_NO_WINDOW (0x08000000)
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        tracing::info!("LlamaRunner: Avviando server invisibile per {:?}", path);

        let child = Command::new("engine/llama-server.exe")
            .arg("-m")
            .arg(path)
            .arg("-ngl")
            .arg("99") // Forza GPU
            .arg("--port")
            .arg("8080")
            .arg("-c")
            .arg("4096") // Context size
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .map_err(|e| anyhow!("Impossibile avviare llama-server.exe: {}", e))?;

        let client = reqwest::blocking::Client::new();

        // Polling /health (Fase 2 - Client-Server)
        // Aspettiamo che il server sia pronto e la VRAM caricata
        let mut ready = false;
        for i in 0..30 { // Timeout 30 secondi
            tracing::info!("LlamaRunner: Polling /health (tentativo {}/30)...", i + 1);
            if let Ok(resp) = client.get("http://127.0.0.1:8080/health").send() {
                if resp.status().is_success() {
                    ready = true;
                    break;
                }
            }
            std::thread::sleep(Duration::from_secs(1));
        }

        if !ready {
            return Err(anyhow!("Il server llama-server non ha risposto in tempo utile."));
        }

        tracing::info!("LlamaRunner: Server pronto e VRAM carica.");
        Ok(Self { child, client })
    }

    /// Genera testo tramite API HTTP (OpenAI Compatible) con streaming.
    pub fn generate(&self, prompt: &str, tx: &Sender<EngineEvent>, rx: &Receiver<EngineCommand>) -> Result<()> {
        let payload = json!({
            "messages": [
                {"role": "user", "content": prompt}
            ],
            "stream": true,
            "temperature": 0.7,
            "max_tokens": 1024
        });

        let response = self.client.post("http://127.0.0.1:8080/v1/chat/completions")
            .json(&payload)
            .send()
            .map_err(|e| anyhow!("Errore richiesta HTTP: {}", e))?;

        let reader = BufReader::new(response);

        for line in reader.lines() {
            // Verifica STOP (Fase 2 - Step 4)
            if let Ok(EngineCommand::Stop) = rx.try_recv() {
                tracing::warn!("LlamaRunner: Ricevuto comando STOP durante streaming.");
                break;
            }

            let line = line.map_err(|e| anyhow!("Errore lettura stream: {}", e))?;
            if line.is_empty() { continue; }
            if line == "data: [DONE]" { break; }

            if let Some(stripped) = line.strip_prefix("data: ") {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(stripped) {
                    if let Some(content) = val["choices"][0]["delta"]["content"].as_str() {
                        let _ = tx.send(EngineEvent::NewToken(content.to_string()));
                    }
                }
            }
        }

        Ok(())
    }
}

/// Implementazione Drop: Stermina il processo zombie alla chiusura di Titan AI.
impl Drop for LlamaRunner {
    fn drop(&mut self) {
        tracing::warn!("LlamaRunner: Terminazione llama-server.exe per rilascio VRAM...");
        let _ = self.child.kill();
        let _ = self.child.wait();
        tracing::info!("LlamaRunner: VRAM rilasciata correttamente.");
    }
}
