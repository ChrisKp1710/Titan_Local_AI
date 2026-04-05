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
    
    // RAM reale in solo-lettura (Bytes -> GB)
    let total_ram_gb = sys.total_memory() as f32 / (1024.0 * 1024.0 * 1024.0);
    
    // Rilevamento GPU e VRAM tramite PowerShell (più moderno di WMIC)
    let (gpu_name, vram_gb) = get_gpu_info_powershell();
    
    tracing::info!("Hardware Detect: RAM: {:.2} GB | GPU: {} | VRAM: {:.2} GB", total_ram_gb, gpu_name, vram_gb);
    
    // Logica di classificazione: con una 7900 XTX e 64GB RAM, sei in Beast Mode assoluta.
    let is_high_end = total_ram_gb >= 31.0 || vram_gb >= 11.0; 
    
    HardwareInfo {
        total_ram_gb,
        vram_gb,
        gpu_name,
        is_high_end,
    }
}

/// Chiama PowerShell per recuperare info moderne sulla GPU (Supera i limiti di WMIC)
fn get_gpu_info_powershell() -> (String, f32) {
    // Comando PowerShell per estrarre nome e memoria
    let ps_cmd = "Get-CimInstance Win32_VideoController | Select-Object Name, AdapterRAM | Format-List";
    
    let output = Command::new("powershell")
        .args(["-Command", ps_cmd])
        .output();

    match output {
        Ok(out) => {
            let res = String::from_utf8_lossy(&out.stdout);
            let mut name = String::new();
            let mut vram_raw = 0u64;

            for line in res.lines() {
                let line = line.trim();
                if line.starts_with("Name") {
                    name = line.split(':').nth(1).unwrap_or("").trim().to_string();
                } else if line.starts_with("AdapterRAM") {
                    if let Ok(val) = line.split(':').nth(1).unwrap_or("").trim().parse::<u64>() {
                        vram_raw = val;
                    }
                }
            }
            
            // Heuristic: Se AdapterRAM riporta ~4GB ma è una 7900 XTX, lo sappiamo che sono di più.
            // Purtroppo Win32_VideoController API è rimasta ai 32-bit.
            // Per ora mostriamo il valore reale rilevato, ma assicuriamo il nome.
            let mut vram_gb = vram_raw as f32 / (1024.0 * 1024.0 * 1024.0);
            
            // Correzione manuale per bug noti delle API Windows su GPU High-End
            if name.contains("7900 XTX") && vram_gb < 20.0 {
                vram_gb = 24.0; // Valore reale della 7900 XTX
            } else if name.contains("4090") && vram_gb < 20.0 {
                vram_gb = 24.0;
            }

            if name.is_empty() { name = "GPU Standard".into(); }
            (name, vram_gb)
        }
        Err(_) => ("GPU Non Rilevata".to_string(), 0.0),
    }
}
