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
    sys.refresh_memory();
    
    // Correzione Calcolo RAM reale (64GB = ~64)
    let total_ram_gb = sys.total_memory() as f32 / (1024.0 * 1024.0 * 1024.0);
    
    // Rilevamento GPU e VRAM stimata via WMIC
    let (gpu_name, vram_gb) = get_gpu_info_windows();
    
    tracing::info!("Hardware Detect: RAM: {:.2} GB | GPU: {} | VRAM: {:.2} GB", total_ram_gb, gpu_name, vram_gb);
    
    let is_high_end = total_ram_gb >= 31.0; 
    
    HardwareInfo {
        total_ram_gb,
        vram_gb,
        gpu_name,
        is_high_end,
    }
}

fn get_gpu_info_windows() -> (String, f32) {
    let output = Command::new("wmic")
        .args(["path", "win32_VideoController", "get", "name,AdapterRAM", "/format:list"])
        .output();

    match output {
        Ok(out) => {
            let res = String::from_utf8_lossy(&out.stdout);
            let mut name = String::new();
            let mut vram = 0.0;

            for line in res.lines() {
                let line = line.trim();
                if line.is_empty() { continue; }
                
                if line.starts_with("Name=") {
                    name = line.replace("Name=", "").trim().to_string();
                } else if line.starts_with("AdapterRAM=") {
                    if let Ok(bytes) = line.replace("AdapterRAM=", "").trim().parse::<u64>() {
                        vram = bytes as f32 / (1024.0 * 1024.0 * 1024.0);
                    }
                }
            }
            
            if name.is_empty() { name = "GPU Standard".into(); }
            (name, vram)
        }
        Err(_) => ("GPU Non Rilevata".to_string(), 0.0),
    }
}
