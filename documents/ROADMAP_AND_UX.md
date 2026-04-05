# Titan Local AI - Roadmap e UI/UX (ROADMAP_AND_UX.md)

## 1. Fasi di Sviluppo (Milestones)
Lo sviluppo deve procedere per fasi rigorose. L'AI non deve proporre codice della Fase 2 se la Fase 1 non è stata completata, testata e approvata.

* **Fase 1: Lo Scheletro (UI Base & Threading)**
    * Setup di `Cargo.toml` con le dipendenze corrette.
    * Creazione del `main.rs` con `eframe/egui`.
    * Implementazione dei canali di comunicazione asincroni (`crossbeam_channel`) tra Main Thread e Worker Thread usando finti messaggi di testo (Dummy Data) per testare i 60fps.
* **Fase 2: Il Motore (Integrazione llama.cpp)**
    * Collegamento dei binding Rust per `llama.cpp`.
    * Caricamento di un piccolo modello `.gguf` di test in memoria.
    * Sostituzione dei messaggi finti con la generazione reale dei token in streaming verso la UI.
* **Fase 3: Gestione Dati, Memoria e Stato**
    * Implementazione del salvataggio della cronologia chat.
    * Gestione dinamica della Context Window: se il limite di token si avvicina, il sistema deve "dimenticare" i messaggi più vecchi per non consumare extra RAM.
* **Fase 4: Rifiniture e Rilascio**
    * Rilevamento hardware dinamico (selezionare in automatico Vulkan o sola CPU).
    * Creazione dello script per la compilazione finale ultra-ottimizzata (Release Build).
    * **Scalabilità Dinamica (High-End Mode):** Implementare la logica che, in presenza di hardware potente (>32GB RAM e GPU dedicata), disattiva i meccanismi di risparmio memoria (come il context-shifting aggressivo) e alloca il 100% dei layer del modello sulla VRAM per massimizzare i Token/s.

## 2. Layout dell'Interfaccia Grafica (UI/UX)
L'interfaccia non deve avere animazioni superflue. Deve essere spartana, pulita e ispirata ai terminali moderni per garantire zero sprechi grafici.

* **Tema Generale:** Dark Mode forzata di default. Zero trasparenze o effetti blur.
* **Pannello Sinistro (Sidebar - 20% della finestra):**
    * In alto: Pulsante "Carica Modello .gguf".
    * Centro: Lista dei modelli disponibili/recenti.
    * In basso: Un minuscolo indicatore in tempo reale delle risorse usate (RAM/VRAM allocata). **Se il sistema rileva hardware High-End, questo indicatore si illumina o cambia stato in "Unleashed/Beast Mode", confermando visivamente all'utente che l'app sta girando al massimo delle prestazioni senza limitatori.**
* **Pannello Centrale (Chat - 80% della finestra):**
    * Area messaggi principale, con barra di scorrimento.
    * Supporto base per il rendering del testo Markdown (grassetto, corsivo, blocchi di codice).
* **Pannello Inferiore (Input Utente):**
    * Casella di testo ancorata in basso (espandibile se si va a capo).
    * Scorciatoie: `Invio` per spedire il messaggio, `Shift + Invio` per andare a capo.
    * Durante l'inferenza, deve apparire un bottone "Stop Generazione" per interrompere il calcolo e liberare il thread.