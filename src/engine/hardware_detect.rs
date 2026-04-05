use sysinfo::System;

/// Informazioni hardware rilevate all'avvio
pub struct HardwareInfo {
    pub total_ram_gb: f32,
    pub is_high_end: bool,
}

/// Rileva le specifiche hardware in modo leggero
pub fn detect_hardware() -> HardwareInfo {
    let mut sys = System::new();
    
    // Rinfresca solo i dati della RAM
    sys.refresh_memory();
    
    let total_ram_kb = sys.total_memory();
    let total_ram_gb = total_ram_kb as f32 / (1024.0 * 1024.0);
    
    tracing::info!("Hardware Detect: RAM Totale rilevata: {:.2} GB", total_ram_gb);
    
    // Logica di classificazione:
    // Se la RAM è >= 32GB, consideriamo il sistema High-End per attivare la Beast Mode.
    // In futuro aggiungeremo il rilevamento della VRAM specifica tramite llama.cpp.
    let is_high_end = total_ram_gb >= 31.0; // Tolleranza per arrotondamenti
    
    if is_high_end {
        tracing::info!("Hardware Detect: Sistema High-End rilevato. Beast Mode Sbloccabile.");
    } else {
        tracing::info!("Hardware Detect: Sistema Standard rilevato. Eco-Mode attiva.");
    }
    
    HardwareInfo {
        total_ram_gb,
        is_high_end,
    }
}
