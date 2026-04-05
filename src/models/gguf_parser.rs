use std::fs::File;
use std::io::Read;
use std::path::Path;
use anyhow::{Result, anyhow};

/// Parser ultra-leggero per file GGUF.
/// Legge solo l'header iniziale senza caricare il file in RAM.
pub struct GgufMetadata {
    pub version: u32,
    pub tensor_count: u64,
    pub metadata_kv_count: u64,
}

pub fn parse_gguf_metadata(path: &Path) -> Result<GgufMetadata> {
    let mut file = File::open(path)?;
    
    // 1. Verifica Magic Number (4 byte: "GGUF")
    let mut magic = [0u8; 4];
    file.read_exact(&mut magic)?;
    
    // GGUF in Hex: 0x47 0x47 0x55 0x46
    if &magic != b"GGUF" {
        return Err(anyhow!("File non valido: Magic Number GGUF non trovato."));
    }

    // 2. Lettura Versione (uint32)
    let mut version_buf = [0u8; 4];
    file.read_exact(&mut version_buf)?;
    let version = u32::from_le_bytes(version_buf);

    // 3. Lettura Tensor Count (uint64)
    let mut tensor_count_buf = [0u8; 8];
    file.read_exact(&mut tensor_count_buf)?;
    let tensor_count = u64::from_le_bytes(tensor_count_buf);

    // 4. Lettura Metadata KV Count (uint64)
    let mut kv_count_buf = [0u8; 8];
    file.read_exact(&mut kv_count_buf)?;
    let metadata_kv_count = u64::from_le_bytes(kv_count_buf);

    // Ci fermiamo qui: non leggiamo i tensori né i valori dei metadati per ora.
    // La RAM è salva.
    
    Ok(GgufMetadata {
        version,
        tensor_count,
        metadata_kv_count,
    })
}
