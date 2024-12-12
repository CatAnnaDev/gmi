#[derive(Debug)]
pub(crate) struct AudioInfo {
    format: String,
    pub(crate) sample_rate: Option<u32>,     // Fréquence d'échantillonnage (en Hz)
    pub(crate) bitrate: Option<u32>,         // Débit binaire (en bps)
    pub(crate) channels: Option<u16>,        // Nombre de canaux
    pub(crate) duration: Option<f64>,        // Durée (en secondes)
    pub(crate) bit_depth: Option<u16>,       // Profondeur de bits (en bits par échantillon)
    pub(crate) codec: Option<String>,        // Codec utilisé
    pub(crate) file_size: Option<u64>,       // Taille du fichier (en octets)
}

impl AudioInfo {
    pub(crate) fn new(format: &str) -> Self {
        AudioInfo {
            format: format.to_string(),
            sample_rate: None,
            bitrate: None,
            channels: None,
            duration: None,
            bit_depth: None,
            codec: None,
            file_size: None,
        }
    }

    pub fn display(&self) {
        println!("\n=== Informations Audio ===");
        println!("Format        : {}", self.format);
        if let Some(codec) = &self.codec {
            println!("Codec         : {}", codec);
        }
        if let Some(sample_rate) = self.sample_rate {
            println!("Sample Rate   : {} Hz", sample_rate);
        }
        if let Some(bitrate) = self.bitrate {
            println!("Bitrate       : {} kbps", bitrate / 1000);
        }
        if let Some(channels) = self.channels {
            println!("Canaux        : {}", channels);
        }
        if let Some(duration) = self.duration {
            println!("Durée         : {:.2} s", duration);
        }
        if let Some(bit_depth) = self.bit_depth {
            println!("Profondeur    : {} bits", bit_depth);
        }
        if let Some(file_size) = self.file_size {
            println!("Taille Fichier: {:.2} MB", file_size as f64 / (1024.0 * 1024.0));
        }
        println!("==========================\n");
    }
}