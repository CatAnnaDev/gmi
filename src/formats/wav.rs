use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::AudioInfo;

pub fn read_wav_info(file: &mut File) -> Option<AudioInfo> {
    let mut buffer = [0u8; 44];
    file.seek(SeekFrom::Start(0)).ok()?;
    file.read_exact(&mut buffer).ok()?;

    if &buffer[0..4] == b"RIFF" && &buffer[8..12] == b"WAVE" {
        let mut info = AudioInfo::new("WAV");

        // Fréquence d'échantillonnage
        let sample_rate = u32::from_le_bytes(buffer[24..28].try_into().ok()?);
        info.sample_rate = Some(sample_rate);

        // Canaux
        let channels = u16::from_le_bytes(buffer[22..24].try_into().ok()?);
        info.channels = Some(channels);

        // Profondeur de bits
        let bit_depth = u16::from_le_bytes(buffer[34..36].try_into().ok()?);
        info.bit_depth = Some(bit_depth);

        // Taille totale des données audio
        let data_size = u32::from_le_bytes(buffer[40..44].try_into().ok()?);
        let file_size = file.metadata().ok()?.len();
        info.file_size = Some(file_size);

        // Durée calculée
        if let Some(sample_rate) = info.sample_rate {
            info.duration = Some(data_size as f64 / (sample_rate as f64 * channels as f64 * (bit_depth as f64 / 8.0)));
        }

        // Codec
        info.codec = Some("PCM".to_string());

        return Some(info);
    }

    None
}

