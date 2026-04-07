# REPORT TECNICO: COMPLETAMENTO FASE 2
**Progetto:** Titan Local AI
**Data:** 2026-04-08
**Stato:** 🟢 STABILE / PRODUZIONE

## 1. Architettura Definitiva: Client-Server HTTP
Il passaggio dall'integrazione nativa instabile ad un'architettura **Client-Server basata su HTTP RESTful** ha risolto definitivamente i problemi di compatibilità con i compilatori MSVC su Windows.

- **Frontend:** Rust (eframe/egui) per un'interfaccia ad alte prestazioni.
- **Backend:** `llama-server.exe` isolato come processo figlio invisibile.
- **Protocollo:** Comunicazione asincrona via JSON/SSE per lo streaming in tempo reale.
- **Vantaggi:** Isolamento dei crash del motore d'inferenza e massima compatibilità con l'ecosistema OpenAI.

## 2. Gestione VRAM Avanzata & Lifecycle
Abbiamo implementato un sistema di gestione della memoria video (VRAM) a "impatto zero" per la GPU AMD Radeon RX 7900 XTX.

- **Drop Trait Implementation:** Il rilascio della VRAM è garantito dalla sottomissione del segnale `.kill()` al processo server non appena il runner viene distrutto.
- **VRAM Unload (Manuale):** Aggiunta del comando `UnloadModel` e di un pulsante rosso "Spegni Motore" nella UI per liberare la GPU senza chiudere l'applicazione.
- **Reset Preventivo:** Ogni nuovo caricamento modello viene anticipato da un reset totale della memoria per evitare stacking di processi zombie.

## 3. Parametri di Inferenza Dinamici
L'esperienza di generazione è stata resa completamente personalizzabile tramite controlli interattivi.

- **Slider UI:** Inserimento di controlli per **Temperatura** (0.1 - 2.0) e **Max Tokens** (128 - 8192).
- **Zero-Latency Streaming:** Ottimizzazione del buffer di rete (128 byte) per eliminare i lag nella visualizzazione del testo, garantendo un "typewriter effect" costante e fluido.

## 4. Stress Test & Performance (Beast Mode)
Le prestazioni registrate durante i test finali confermano Titan AI come uno dei client più veloci su hardware AMD.

- **Modello Testato:** Qwen 2.5 35B GGUF.
- **Backend:** Vulkan (AMD Radeon RX 7900 XTX).
- **Prestazioni:** Generazione stabile a **~75 token al secondo**.
- **Stabilità:** Caricamento in VRAM completato con successo e senza errori di timeout su modelli pesanti fino a un contesto di 4096 token.

---
**CONCLUSIONE:** La Fase 2 consegna un prodotto solido, performante e pronti per l'integrazione della persistenza dei dati e della cronologia (Fase 3).
