use std::io::{BufRead, Seek, SeekFrom};

use crate::AudioInfo;
use crate::gmi_error::GMIResult;

pub fn read_wav_info<R: BufRead + Seek>(reader: &mut R) -> GMIResult<AudioInfo> {
    let mut buffer = [0u8; 44];
    reader.seek(SeekFrom::Start(0)).ok();
    reader.read_exact(&mut buffer).ok();

        let mut info = AudioInfo::new("WAV");

        // Fréquence d'échantillonnage
        let sample_rate = u32::from_le_bytes(buffer[24..28].try_into().ok().unwrap());
        info.sample_rate = Some(sample_rate);

        // Canaux
        let channels = u16::from_le_bytes(buffer[22..24].try_into().ok().unwrap());
        info.channels = Some(channels);

        // Profondeur de bits
        let bit_depth = u16::from_le_bytes(buffer[34..36].try_into().ok().unwrap());
        info.bit_depth = Some(bit_depth);

        // Taille totale des données audio
        let data_size = u32::from_le_bytes(buffer[40..44].try_into().ok().unwrap());

        // Durée calculée
        if let Some(sample_rate) = info.sample_rate {
            info.duration = Some(data_size as f64 / (sample_rate as f64 * channels as f64 * (bit_depth as f64 / 8.0)));
        }

        // Codec
        info.codec = Some("PCM".to_string());

        Ok(info)
}

pub fn matches(header: &[u8]) -> bool{
    header.starts_with(b"RIFF") && &header[8..12] == b"WAVE"
}

