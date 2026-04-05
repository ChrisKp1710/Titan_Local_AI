use sysinfo::System;
use std::process::Command;

/// Informazioni hardware rilevate all'avvio
pub struct HardwareInfo {
    pub total_ram_gb: f32,
    pub ram_model: String,
    pub vram_gb: f32,
    pub gpu_name: String,
    pub cpu_name: String,
    pub cpu_cores: usize,
    pub cpu_threads: usize,
    pub cpu_voltage: f32,
    pub is_high_end: bool,
}

/// Rileva le specifiche hardware in modo leggero
pub fn detect_hardware() -> HardwareInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    // RAM reale (Bytes -> GB)
    let total_ram_gb = sys.total_memory() as f32 / (1024.0 * 1024.0 * 1024.0);
    
    // Dati RAM Fisica via PowerShell (Manufacturer + Speed)
    let ram_model = get_ram_info_powershell();
    
    // Dati CPU via sysinfo (strutturali) e PowerShell (Nome)
    let cpu_cores = sys.physical_core_count().unwrap_or(0);
    let cpu_threads = sys.cpus().len();
    let cpu_name = get_cpu_name_powershell();
    
    // Dati GPU via PowerShell
    let (gpu_name, vram_gb) = get_gpu_info_powershell();
    
    // Dati Voltaggio via PowerShell
    let cpu_voltage = get_cpu_voltage_powershell();
    
    tracing::info!("Hardware Detect: RAM: {:.2} GB ({}) | CPU: {} | GPU: {}", 
        total_ram_gb, ram_model, cpu_name, gpu_name);
    
    let is_high_end = total_ram_gb >= 31.0 || (vram_gb >= 11.0 && cpu_cores >= 8); 
    
    HardwareInfo {
        total_ram_gb,
        ram_model,
        vram_gb,
        gpu_name,
        cpu_name,
        cpu_cores,
        cpu_threads,
        cpu_voltage,
        is_high_end,
    }
}

fn get_ram_info_powershell() -> String {
    let output = Command::new("powershell")
        .args(["-Command", "Get-CimInstance Win32_PhysicalMemory | Select-Object Manufacturer, Speed | Format-List"])
        .output();

    if let Ok(out) = output {
        let res = String::from_utf8_lossy(&out.stdout);
        let mut manufacturer = String::new();
        let mut speed = String::new();

        for line in res.lines() {
            let line = line.trim();
            if line.starts_with("Manufacturer") {
                manufacturer = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.starts_with("Speed") {
                speed = line.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        }
        
        if manufacturer.is_empty() {
            "DIMM Standard".to_string()
        } else {
            format!("{} {} MHz", manufacturer, speed)
        }
    } else {
        "RAM Rilevata".to_string()
    }
}

fn get_cpu_name_powershell() -> String {
    let output = Command::new("powershell")
        .args(["-Command", "Get-CimInstance Win32_Processor | Select-Object -ExpandProperty Name"])
        .output();

    if let Ok(out) = output {
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    } else {
        "AMD Processor".to_string()
    }
}

fn get_gpu_info_powershell() -> (String, f32) {
    let ps_cmd = "Get-CimInstance Win32_VideoController | Select-Object Name, AdapterRAM | Format-List";
    let output = Command::new("powershell").args(["-Command", ps_cmd]).output();

    if let Ok(out) = output {
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
        
        let mut vram_gb = vram_raw as f32 / (1024.0 * 1024.0 * 1024.0);
        if (name.contains("7900") || name.contains("4090") || name.contains("3090")) && vram_gb < 12.0 {
            vram_gb = 24.0; 
        }
        (name, vram_gb)
    } else {
        ("GPU Non Rilevata".to_string(), 0.0)
    }
}

fn get_cpu_voltage_powershell() -> f32 {
    let ps_cmd = "Get-CimInstance Win32_Processor | Select-Object -ExpandProperty CurrentVoltage";
    let output = Command::new("powershell").args(["-Command", ps_cmd]).output();

    if let Ok(out) = output {
        let res = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if let Ok(val) = res.parse::<f32>() {
            if val > 5.0 { val / 10.0 } else { val }
        } else { 0.0 }
    } else { 0.0 }
}
