# Architettura Software Titan Local AI (Titan AI)

## 1. Visione e Motivazione (Il "Perché")
L'ecosistema attuale dell'AI locale (es. LM Studio) è appesantito da framework web come Electron. Questo divora memoria vitale (RAM), penalizzando macchine datate o con risorse limitate (8-16GB di RAM), e mandando il sistema in swap. 

**Obiettivo del progetto:** Costruire un client di chat e agenti AI locale che sia un "carro armato" di efficienza. Democratizzare l'AI locale garantendo:
- **Zero sprechi di RAM:** L'interfaccia non deve consumare più di 50MB.
- **Massima velocità:** Accesso diretto all'hardware (CPU/Vulkan/ROCm) senza colli di bottiglia software.
- **Accessibilità:** Fluidità garantita anche su PC datati (5-7 anni).
- **Modularità Estrema:** Architettura a plugin/moduli per un'estensione semplificata.
- **Scalabilità Dinamica verso l'Alto:** Il risparmio risorse non è fine a se stesso. Su macchine High-End, i Gigabyte di RAM e VRAM risparmiati non usando Electron vengono dirottati per caricare modelli più grandi o per massimizzare il batch size e la velocità di inferenza, senza alcun collo di bottiglia artificiale.

---

## 2. Regole d'Ingaggio per lo Sviluppo (CRITICO PER L'AI)
Questo progetto segue regole rigorose per mantenere un footprint di memoria minuscolo. Ogni riga di codice suggerita deve rispettare questi principi:
1. **Zero-Cost Abstractions:** Preferire sempre strutture dati native di Rust. Evitare la clonazione non necessaria di stringhe (`.clone()`). Usare reference e `Cow` (Clone-on-Write) dove possibile.
2. **Concorrenza Rigida:** L'interfaccia UI (`egui`) gira sul Main Thread e deve girare a 60fps. L'inferenza del modello gira su un Worker Thread dedicato. I due thread comunicano ESCLUSIVAMENTE tramite canali asincroni (`mpsc` o `crossbeam_channel`). Il Main Thread non deve MAI aspettare (bloccarsi) per l'Engine.
3. **Gestione Errori Gracile:** Il programma non deve mai andare in "panic". Usare sempre `Result` e il crate `anyhow` per propagare gli errori alla UI, informando l'utente se l'hardware non supporta una funzione.
4. **Dipendenze Minime:** Non aggiungere crate enormi se serve solo una piccola funzione. Selezionare attentamente le feature flag nel `Cargo.toml`.

---

## 3. Stack Tecnologico e Crate Scelti
- **Linguaggio:** Rust (edizione 2021).
- **Frontend (UI):** `egui` via `eframe` (Immediate mode GUI, compilata nativamente).
- **Backend (Inferenza):** Binding Rust per `llama.cpp` (es. `llama-cpp-2` o `llama_rs`).
- **Concorrenza/Canali:** `crossbeam_channel` (per comunicazione UI-Engine rapida).
- **Serializzazione (Config):** `serde` e `serde_json`.
- **Log e Debug:** `tracing` e `tracing-subscriber`.

---

## 4. Struttura del Progetto (Architettura Modulare)

```text
titan_ai/
│
├── Cargo.toml                 # Dipendenze con feature flags ben definite
├── README.md                  # Introduzione al progetto
├── ARCHITECTURE.md            # Questo documento contestuale
│
├── src/
│   ├── main.rs                # Inizializza `eframe`, i canali mpsc, e avvia i thread.
│   ├── app_state.rs           # Struct centrale condivisa (Rx/Tx channels, status flag).
│   │
│   ├── ui/                    # MODULO FRONTEND (Nessuna logica pesante qui)
│   │   ├── mod.rs             
│   │   ├── main_window.rs     # Rendering loop a 60fps
│   │   ├── chat_panel.rs      # Gestisce la visualizzazione dello stream di token
│   │   ├── model_sidebar.rs   
│   │   ├── settings_view.rs   
│   │   └── downloader_ui.rs   
│   │   
│   ├── engine/                # MODULO BACKEND (Heavy lifting, gira in background)
│   │   ├── mod.rs
│   │   ├── llm_runner.rs      # Inizializza il contesto llama.cpp e gestisce la sessione
│   │   ├── memory_mmap.rs     # Regole di allocazione (mmap)
│   │   ├── token_stream.rs    # Invia i token generati al canale Tx verso la UI
│   │   └── hardware_detect.rs # Fallback automatico: Vulkan -> OpenBLAS -> CPU nativa
│   │
│   ├── models/                # MODULO GESTIONE DATI
│   │   ├── mod.rs
│   │   ├── gguf_parser.rs     # Lettura veloce degli header senza caricare l'intero file
│   │   ├── downloader.rs      # Download asincrono con ripresa in caso di errore
│   │   └── local_storage.rs   
│   │
│   ├── agents/                # MODULO LOGICA AI
│   │   ├── mod.rs
│   │   ├── system_prompts.rs  
│   │   ├── chat_history.rs    # Gestione dinamica del Context Size (taglio vecchi messaggi)
│   │   └── tool_caller.rs     
│   │
│   └── utils/                 # MODULO UTILITY
│       ├── mod.rs
│       ├── logger.rs          
│       └── formatting.rs      # Parser Markdown custom leggero per egui
```

---

## 5. Flusso di Dati Esatto (Data Flow UI <-> Engine)
Per evitare colli di bottiglia, il flusso è strettamente asincrono:

1. **Input:** L'utente preme "Invia" in `chat_panel.rs`.
2. **Comando:** La UI invia un messaggio (es. `Command::Generate("Ciao")`) attraverso il canale Tx al Thread dell'Engine.
3. **Fluidità:** Il Thread UI continua il rendering fluido, mostrando "Sto pensando...".
4. **Elaborazione:** L'Engine (`llm_runner.rs`) elabora la richiesta.
5. **Streaming:** Per ogni parola generata, l'Engine invia un evento (es. `Event::NewToken("Ciao")`) sul canale di ritorno.
6. **Update:** Il Thread UI riceve l'evento nel suo loop di aggiornamento e accoda la parola a schermo istantaneamente.
