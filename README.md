# 🛸 TITAN LOCAL AI
**Bare-Metal LLM Inference Engine | Build: Rust & DXGI**

![Language](https://img.shields.io/badge/Language-Rust-orange.svg)
![Platform](https://img.shields.io/badge/Platform-Windows_11-blue.svg)
![Graphics](https://img.shields.io/badge/Graphics-DXGI%20/%20DirectX-informational.svg)
![Performance](https://img.shields.io/badge/Performance-Beast_Mode-red.svg)
![License](https://img.shields.io/badge/License-GPLv3-yellow.svg)

---

<p align="center">
  <img src="https://kodechris.dev/favicon.ico" width="48" height="48">
  <br>
  Developed by <a href="https://kodechris.dev/"><b>Christian Koscielniak Pinto</b></a>
  <br>
  <i>Zero Waste. Direct Access. Extreme Efficiency.</i>
</p>

---

Benvenuto nel portale di sviluppo di **Titan Local AI**. Questo repository non è solo un client, ma un manifesto di efficienza computazionale. Rifiutiamo l'overhead di Electron e la ridondanza dei processi in eccesso. Qui la RAM è gestita come una risorsa sacra e l'accesso all'hardware è diretto (Bare-Metal).

---

## 🚦 STATO DEL PROGETTO
| Fase | Titolo | Stato | Registro Tecnico |
| :--- | :--- | :--- | :--- |
| **01** | **Lo Scheletro (UI & Threading)** | `✅ COMPLETATA` | [FASE1.md](FASE1.md) |
| **02** | **Engine Client-Server & GPU Offload** | `✅ COMPLETATA` | [FASE_2_COMPLETATA.md](FASE_2_COMPLETATA.md) |
| **03** | **Streaming Asincrono & Reasoning Parser** | `🏗️ IN CORSO` | *In fase di sviluppo* |
| **04** | **Chat History & UI Refinement** | `⏳ PIANIFICATA` | - |

---

## ✅ TRAGUARDI RAGGIUNTI: FASE 2
Abbiamo consolidato il cuore pulsante di Titan:
- [x] **Integrazione Motore IA**: Migrazione riuscita verso `llama-server.exe`.
- [x] **Architettura Client-Server**: Comunicazione HTTP RESTful isolata e sicura.
- [x] **Offload GPU Totale**: Supporto Vulkan per GPU AMD (RX 7900 XTX).
- [x] **Gestione VRAM**: Implementazione del tratto `Drop` e tasto "Spegni Motore" per pulizia istantanea.
- [x] **Inference Sliders**: Controllo dinamico di Temperatura e Max Tokens dalla UI.

---

## 🏗️ CANTIERE ATTUALE: FASE 3
Obiettivi prioritari per l'ottimizzazione dell'esperienza utente:
- **Streaming Zero-Latency Reale**: Migrazione a uno stream asincrono a basso livello per eliminare ogni micro-freeze della UI durante l'inferenza di modelli massicci (35B+).
- **Parser "Reasoning" (LM Studio Style)**: Logica di intercettazione dei tag `<think>...</think>`. Visualizzazione dinamica del pensiero del modello (corsivo grigio) con sistema di auto-collasso alla ricezione della risposta finale.
- **Persistent Chat History**: Salvataggio locale delle sessioni di chat.

---

## 📜 FILOSOFIA DI SVILUPPO
1. **Direct Access**: Se esiste un'API di sistema, la usiamo preferendola a qualsiasi wrapper.
2. **Deterministic UI**: L'interfaccia non deve mai attendere l'Engine. 60fps garantiti.
3. **Zero Waste**: Ogni byte di RAM deve essere giustificato.
4. **Extreme Modularity**: Moduli piccoli, sicuri e focalizzati.

---

## 🛠️ STACK TECNOLOGICO (V1.2)
- **Core Logic**: Rust 1.75+
- **Inference Server**: Llama.cpp (Vulkan Backend)
- **Networking**: `reqwest` (HTTP/1.1 Streaming)
- **UI Framework**: `eframe` / `egui` (GLOW Backend)
- **Monitoring**: `sysinfo`, `windows-rs` (DXGI Interface)

---

## 📄 LICENZA
Questo progetto è rilasciato sotto la licenza **GPLv3**.

---

*Documentazione aggiornata al: Aprile 2026*
