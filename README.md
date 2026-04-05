# 🛸 TITAN LOCAL AI
**Bare-Metal LLM Inference Engine | Build: Rust & DXGI**

![Language](https://img.shields.io/badge/Language-Rust-orange.svg)
![Platform](https://img.shields.io/badge/Platform-Windows_11-blue.svg)
![Graphics](https://img.shields.io/badge/Graphics-DXGI%20/%20DirectX-informational.svg)
![Performance](https://img.shields.io/badge/Performance-Beast_Mode-red.svg)
![License](https://img.shields.io/badge/License-MIT-yellow.svg)

Benvenuto nel portale di sviluppo di **Titan Local AI**. Questo repository non è solo un client, ma un manifesto di efficienza computazionale. Rifiutiamo l'overhead di Electron e la ridondanza dei processi in eccesso. Qui la RAM è gestita come una risorsa sacra e l'accesso all'hardware è diretto (Bare-Metal).

---

## 👨‍💻 DEVELOPED BY
### [**Christian Koscielniak Pinto**](https://kodechris.dev/)
*Frontend Developer & UI/UX Designer*

[![Portfolio](https://img.shields.io/badge/Visit_Portfolio-kodechris.dev-blue.svg?logo=googlechrome&logoColor=white)](https://kodechris.dev/)

---

## 🚦 STATO DEL PROGETTO
| Fase | Titolo | Stato | Registro Tecnico |
| :--- | :--- | :--- | :--- |
| **01** | **Lo Scheletro (UI & Threading)** | `✅ COMPLETATA` | [FASE1.md](FASE1.md) |
| **02** | **GGUF Loader & Memory MMap** | `🏗️ IN CORSO` | *In fase di stesura* |
| **03** | **Llama.cpp Integration** | `⏳ PIANIFICATA` | - |
| **04** | **Beast Mode Optimizations** | `⏳ PIANIFICATA` | - |

---

## 🏗️ CANTIERE ATTUALE: FASE 2
Siamo nel cuore della gestione della memoria. In questa fase stiamo implementando:
- **Native File Dialog**: Integrazione tramite `rfd` per un'esperienza desktop coerente.
- **GGUF Parser**: Accesso istantaneo all'header e ai metadati del modello.
- **Memory Mapping**: Mapping virtuale dei file tramite `memmap2` per caricamenti a latenza zero.

---

## 📜 FILOSOFIA DI SVILUPPO
1. **Direct Access**: Se esiste un'API di sistema, la usiamo preferendola a qualsiasi wrapper.
2. **Deterministic UI**: L'interfaccia non deve mai attendere l'Engine. 60fps garantiti.
3. **Zero Waste**: Ogni byte di RAM deve essere giustificato.
4. **Extreme Modularity**: Moduli piccoli, sicuri e focalizzati (Filosofia "Titan").

---

## 🛠️ STACK TECNOLOGICO (V1.0)
- **Core Logic**: Rust 1.75+
- **UI Framework**: `eframe` / `egui` (Backend GLOW)
- **Concurrency**: `crossbeam-channel` (Asynchronous Message Passing)
- **Direct System Access**: `windows-rs` (DXGI Interface), `sysinfo` (Selective Hardware Fetching)

---

## 📄 LICENZA
Questo progetto è rilasciato sotto la licenza **MIT**. Consulta il file [LICENSE](LICENSE) per i termini completi.

---

*Documentazione aggiornata al: Aprile 2026*
