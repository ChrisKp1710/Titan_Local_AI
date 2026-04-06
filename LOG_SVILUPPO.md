# LOG SVILUPPO - Titan Local AI
**Data Ultimo Aggiornamento:** 2026-04-06

## 🚀 Stato Attuale: Architettura Client-Server
Abbiamo migrato con successo il motore di inferenza da un'integrazione nativa (`llama-cpp-2`) a un'architettura **Client-Server invisibile**. Questo risolve i problemi di compatibilità MSVC e garantisce stabilità nel caricamento dei modelli GGUF.

- **Motore:** `llama-server.exe` (OpenAI Compatible API).
- **Comunicazione:** Chiamate HTTP tramite il crate `reqwest` (modalità blocking con streaming).
- **Integrazione UI:** I token vengono ricevuti via streaming JSON e inviati alla sidebar tramite canali asincroni.

## 📂 Stato dei File e Dipendenze
- Il binario `llama-server.exe` e le sue DLL critiche (inclusa la risoluzione di `mtmd.dll`) sono correttamente posizionati nella cartella `/engine`.
- Il sistema utilizza correttamente le API **Vulkan** per l'offload totale su GPU AMD Radeon RX 7900 XTX.

## 🛠 Configurazione di Debug & Diagnostica (Attiva)
Per facilitare il fine-tuning e la risoluzione dei problemi di caricamento VRAM, sono state applicate le seguenti configurazioni temporanee in `src/engine/llm_runner.rs`:

- **Visibilità Log:** Il flag `CREATE_NO_WINDOW` è stato commentato. All'avvio di un modello, viene mostrata la finestra del terminale di `llama-server` per monitorare i log di sistema.
- **Timeout Esteso:** Il polling sull'endpoint `/health` è stato elevato a **120 tentativi (2 minuti)** per permettere il caricamento di modelli pesanti (es. Qwen 2.5 35B) sulla VRAM senza restituire errori prematuri alla UI.

## 🧠 Logica di Lifecycle & Gestione Memoria
- **Safe Cleanup:** Implementato il tratto `Drop` per la struct `LlamaRunner`. Alla chiusura dell'applicazione o al cambio modello, il processo `llama-server.exe` viene terminato forzatamente (`.kill()`), garantendo il rilascio immediato della VRAM.
- **Parametri Beast Mode:** Il server viene avviato con `-ngl 99`, `-c 4096` e `--alias model_1`.

## 📅 Roadmap per la Prossima Sessione
1. **Parametrizzazione UI:** Implementare slider nella sidebar per controllare `temperature`, `top_p` e `context_size` dinamicamente.
2. **Restore Invisibility:** Una volta confermata la stabilità totale, riattivare `CREATE_NO_WINDOW` per nascondere il terminale all'utente finale.
3. **Ottimizzazione Prompt:** Affinare il template dei messaggi per migliorare la coerenza delle risposte di Titan AI.

---
*Punto di ripristino stabilizzato. Caricamento VRAM e inferenza streaming operativi.*
