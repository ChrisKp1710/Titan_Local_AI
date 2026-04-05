# TITAN LOCAL AI (Titan AI)
**Architettura Bare-Metal per Inferenza LLM Ultra-Efficiente**

Titan AI è un client desktop per Large Language Models (LLM) progettato per eliminare ogni spreco di risorse. Rifiutiamo categoricamente framework pesanti come Electron: in Titan AI, la **RAM è sacra**. Il software è scritto interamente in Rust per garantire prestazioni "vicine al metallo" e un avvio istantaneo.

---

## 🚀 STATO DELLO SVILUPPO
> **FASE 1: LO SCHELETRO (UI Base & Threading) — [COMPLETATA]**

Il nucleo fondamentale del sistema è operativo. La comunicazione asincrona e il rilevamento hardware nativo sono stati validati e consolidati.

---

## 🛠️ RISULTATI FASE 1 (Changelog Tecnico)

### 1. Architettura Multi-Thread Rigorosa
- **UI Isolata**: Il Main Thread gestisce esclusivamente il rendering della UI a 60fps costanti, garantendo zero latenza nell'interazione tramite il crate `eframe`.
- **Engine Worker**: Un thread worker dedicato (`std::thread::spawn`) gestisce la logica di inferenza e il caricamento dei modelli senza bloccare l'interfaccia.
- **Comunicazione Asincrona**: Lo scambio di messaggi tra UI ed Engine avviene tramite canali isolati `crossbeam-channel` per massima sicurezza e velocità.

### 2. Rendering Nativo & Spartan UI
- **Backend Glow**: Interfaccia renderizzata tramite WebGL/OpenGL nativo (glow) per la massima compatibilità hardware senza overhead software.
- **Design Spartan**: Interfaccia monospazio scura focalizzata sull'efficienza visiva e sulla riduzione del footprint di memoria.

### 3. Rilevamento Hardware Bare-Metal
- **Nativo DXGI**: Integrazione diretta alle Windows API (`IDXGIFactory1`) per estrarre il nome della GPU e la VRAM a 64-bit, superando i limiti storici di 4GB delle API WMI/WMIC.
- **Ottimizzazione Sysinfo**: Rilevamento RAM e CPU iper-ottimizzato tramite caricamento selettivo dei metadati di sistema per un avvio in microsecondi.
- **Logica Dinamica "Beast Mode"**: Sistema dinamico di flagging (`is_high_end`) che sblocca ottimizzazioni per hardware con >32GB RAM e >11GB VRAM.

---

## 📦 STACK TECNOLOGICO
Il progetto si basa su crate Rust selezionati per il loro minimo impatto e massima affidabilità:
- **eframe / egui**: Framework UI immediato e leggero.
- **crossbeam-channel**: Concorrenza multi-produttore/multi-consumatore ultra-rapida.
- **sysinfo**: Accesso selettivo ai dati di sistema.
- **windows**: Interfacciamento diretto con le API di sistema di Microsoft (DXGI).
- **anyhow / tracing**: Gestione degli errori e logging strutturato ad alte prestazioni.

---

## 🏗️ PROSSIMI PASSI: FASE 2
Il cantiere si sposta ora sul motore di inferenza:
1. **GGUF Metadata Parser**: Lettura istantanea dei parametri e dei layer del modello senza caricamento integrale in RAM.
2. **Memory Mapping (MMap)**: Implementazione del caricamento dinamico dei modelli tramite memoria virtuale.
3. **Integrazione llama.cpp**: Collegamento dei binding nativi per l'inferenza hardware-accelerata (CPU/Vulkan/ROCm).

---

*"Zero sprechi. Massima velocità. Accesso Diretto."*
