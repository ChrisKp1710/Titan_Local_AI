use sysinfo::System;
use std::process::Command;

/// Informazioni hardware rilevate all'avvio
pub struct HardwareInfo {
    pub total_ram_gb: f32,
    pub vram_gb: f32,
    pub gpu_name: String,
    pub is_high_end: bool,
}

/// Rileva le specifiche hardware in modo leggero
pub fn detect_hardware() -> HardwareInfo {
    let mut sys = System::new();
    
    // Rinfresca solo i dati della RAM
    sys.refresh_memory();
    
    // Correzione Calcolo RAM: sysinfo 0.30 restituisce BYTES
    let total_ram_gb = sys.total_memory() as f32 / (1024.0 * 1024.0 * 1024.0);
    
    // Rilevamento GPU e VRAM stimata via WMIC
    let (gpu_name, vram_gb) = get_gpu_info_windows();
    
    tracing::info!("Hardware Detect: RAM: {:.2} GB | GPU: {} | VRAM: {:.2} GB", total_ram_gb, gpu_name, vram_gb);
    
    // Logica di classificazione:
    // Se la RAM è >= 32GB, consideriamo il sistema High-End per attivare la Beast Mode.
    let is_high_end = total_ram_gb >= 31.0; 
    
    HardwareInfo {
        total_ram_gb,
        vram_gb,
        gpu_name,
        is_high_end,
    }
}

/// Chiama WMIC (Windows Management Instrumentation) per recuperare Info GPU
fn get_gpu_info_windows() -> (String, f32) {
    let output = Command::new("wmic")
        .args(["path", "win32_VideoController", "get", "name,AdapterRAM", "/format:list"])
        .output();

    match output {
        Ok(out) => {
            let res = String::from_utf8_lossy(&out.stdout);
            let mut name = "Generic GPU".to_string();
            let mut vram = 0.0;

            for line in res.lines() {
                let line = line.trim();
                if line.starts_with("Name=") {
                    name = line.replace("Name=", "").to_string();
                } else if line.starts_with("AdapterRAM=") {
                    if let Ok(bytes) = line.replace("AdapterRAM=", "").parse::<u64>() {
                        // Alcune GPU moderne riportano valori AdapterRAM errati via WMIC (es. u32::MAX),
                        // ma per molte schede funziona bene come prima stima.
                        vram = bytes as f32 / (1024.0 * 1024.0 * 1024.0);
                    }
                }
            }
            (name, vram)
        }
        Err(_) => ("Generic GPU".to_string(), 0.0),
    }
}
