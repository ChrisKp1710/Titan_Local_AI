use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind};
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory1, IDXGIFactory1, IDXGIAdapter1, DXGI_ERROR_NOT_FOUND,
};
use windows::core::Result;

/// Informazioni hardware rilevate all'avvio
pub struct HardwareInfo {
    pub total_ram_gb: f32,
    pub ram_model: String,
    pub vram_gb: f32,
    pub gpu_name: String,
    pub cpu_name: String,
    pub cpu_cores: usize,
    pub cpu_threads: usize,
    pub is_high_end: bool,
}

/// Rileva le specifiche hardware in modo bare-metal e ultra-veloce
pub fn detect_hardware() -> HardwareInfo {
    // 1. Ottimizzazione sysinfo: caricamento selettivo
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_memory(MemoryRefreshKind::new().with_ram())
            .with_cpu(CpuRefreshKind::new())
    );
    
    // Refresh minimo necessario
    sys.refresh_memory();
    sys.refresh_cpu_usage();
    
    let total_ram_gb = sys.total_memory() as f32 / (1024.0 * 1024.0 * 1024.0);
    
    // Nome CPU: sysinfo 0.30 popola il brand dopo il refresh
    let cpu_name = sys.cpus().first()
        .map(|c| c.brand().trim().to_string())
        .unwrap_or_else(|| "CPU Sconosciuta".to_string());
        
    let cpu_cores = sys.physical_core_count().unwrap_or(0);
    let cpu_threads = sys.cpus().len();
    
    // 2. DXGI: Rilevamento GPU Bare-Metal
    let (gpu_name, vram_gb) = get_gpu_info_dxgi().unwrap_or_else(|_| ("GPU Non Rilevata".to_string(), 0.0));
    
    // 3. Modello RAM Fisica (Best effort via Sysinfo o placeholder per velocità)
    // Per ora manteniamo una stringa pulita, in futuro potremmo usare WinAPI per i banchi SPD.
    let ram_model = "DDR4/DDR5 System RAM".to_string(); 
    
    tracing::info!("Hardware Detect Bare-Metal: CPU: {} ({}C/{}T) | RAM: {:.2} GB | GPU: {} | VRAM: {:.2} GB", 
        cpu_name, cpu_cores, cpu_threads, total_ram_gb, gpu_name, vram_gb);
    
    // Logica High-End potenziata
    let is_high_end = total_ram_gb >= 31.0 || (vram_gb >= 11.0 && cpu_cores >= 8); 
    
    HardwareInfo {
        total_ram_gb,
        ram_model,
        vram_gb,
        gpu_name,
        cpu_name,
        cpu_cores,
        cpu_threads,
        is_high_end,
    }
}

/// Estrae Info GPU via DXGI interfacciandosi direttamente ai driver Windows
fn get_gpu_info_dxgi() -> Result<(String, f32)> {
    unsafe {
        // Creazione Factory DXGI
        let factory: IDXGIFactory1 = CreateDXGIFactory1()?;
        
        // Prendiamo il primo adattatore (GPU Primaria)
        let adapter: IDXGIAdapter1 = factory.EnumAdapters1(0).map_err(|e| {
            if e.code() == DXGI_ERROR_NOT_FOUND.into() {
                // Nessun adattatore trovato
                e
            } else {
                e
            }
        })?;
        
        // Estrazione descrizione adattatore
        let desc = adapter.GetDesc1()?;
        
        // Conversione Nome (UTF-16 -> String)
        let name = String::from_utf16_lossy(&desc.Description)
            .trim_matches(char::from(0))
            .to_string();
            
        // VRAM precisa in byte (DedicatedVideoMemory)
        let vram_gb = desc.DedicatedVideoMemory as f32 / (1024.0 * 1024.0 * 1024.0);
        
        Ok((name, vram_gb))
    }
}
