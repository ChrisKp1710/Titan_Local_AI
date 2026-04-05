# Titan Local AI - Manifesto e Visione del Progetto (VISION.md)

## 1. Il Contesto: Perché stiamo costruendo Titan AI?
Attualmente, il mondo dell'AI locale è dominato da interfacce grafiche bellissime ma estremamente pesanti (come LM Studio o GPT4All). Questi software utilizzano framework web (Electron) che, di base, consumano tra i 500MB e 1GB di RAM solo per mostrare l'interfaccia a schermo.
Per uno sviluppatore con 64GB di RAM e 24GB di VRAM, questo è un fastidio. Ma per un utente normale o un collega con un PC aziendale standard (8-16GB di RAM, GPU integrata), quel Gigabyte sprecato è letale: costringe Windows o Linux a usare il file di Swap sul disco, uccidendo le prestazioni del modello AI e rendendo il PC inutilizzabile.

## 2. La Nostra Missione
Vogliamo **democratizzare l'accesso ai Large Language Models (LLM) in locale**. 
L'obiettivo di Titan AI non è avere l'interfaccia più animata o complessa del mercato. L'obiettivo è creare uno strumento brutale, puro ed estremamente ottimizzato. Un "carro armato" software che faccia una sola cosa e la faccia al massimo della velocità fisica consentita dall'hardware.
Vogliamo permettere a chiunque, anche con un portatile di 5 anni fa, di caricare un modello `.gguf` ottimizzato e avere un assistente AI fluido e reattivo, totalmente offline e privato.

## 3. L'Utente Tipo e l'Hardware Target
Ogni decisione di sviluppo deve tenere a mente il nostro utente peggiore (Worst-Case Scenario):
- **RAM:** 8GB (di cui 4GB già presi dal Sistema Operativo).
- **GPU:** Assente o Integrata (Intel Iris, vecchie APU AMD).
- **Competenza Tecnica:** Media. L'utente non vuole aprire il terminale, vuole un `.exe` o un eseguibile Linux che funzioni al doppio clic, selezioni il file `.gguf` e inizi a chattare.
**ECCEZIONE HIGH-END (Scalabilità Dinamica):** Sebbene progettato per il Worst-Case Scenario, il software NON deve mai limitare artificialmente l'hardware. Se il sistema rileva risorse abbondanti (es. 32-64GB di RAM, 24GB di VRAM), il software deve rimuovere i "limitatori di sopravvivenza". Il risparmio estremo della UI (zero Electron) non serve più a sopravvivere, ma a **dominare**: il 100% delle risorse risparmiate dall'interfaccia deve essere iniettato nel modello AI (Vulkan/CUDA) per spingere i Token-per-Secondo al massimo teorico, superando in velocità competitor come LM Studio.

## 4. I Principi Guida dello Sviluppo (Da applicare sempre)
Se tu, AI, ti trovi a dover scegliere tra due implementazioni, devi **sempre** seguire questa gerarchia di valori:

1. **La RAM è Sacra:** Risparmiare 50MB di RAM è sempre meglio che avere un'animazione fluida nell'interfaccia.
2. **Accesso Diretto (Bare Metal):** Evitare livelli di astrazione inutili. Dobbiamo parlare il più vicino possibile al metallo (CPU, Vulkan per AMD, CUDA).
3. **Fail-Safe (Nessun blocco silente):** Se il PC non ha abbastanza RAM per caricare un modello, il programma non deve crashare. Deve avvisare l'utente gentilmente ("RAM insufficiente, prova un modello più piccolo").
4. **Leggerezza del Codice:** Non importare una libreria gigantesca (es. `tokio` full-features) se ci serve solo un piccolo canale di comunicazione asincrono.

## 5. Il Risultato Atteso
Alla fine dello sviluppo, avremo un singolo file eseguibile (es. `titan-ai.exe`) che pesa pochi megabyte. Quando l'utente lo apre, l'interfaccia si carica istantaneamente consumando ~30MB di RAM. Il resto delle risorse del computer sarà dedicato esclusivamente a `llama.cpp` per generare la risposta più veloce possibile.