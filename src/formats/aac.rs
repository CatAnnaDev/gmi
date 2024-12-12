use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use crate::AudioInfo;

pub fn read_aac_info(file: &mut File) -> Option<AudioInfo> {
    let mut buffer = [0u8; 7];
    file.seek(SeekFrom::Start(0)).ok()?;
    file.read_exact(&mut buffer).ok()?;

    if &buffer[0..2] == b"\xFF\xF1" || &buffer[0..2] == b"\xFF\xF9" {

        let mut info = AudioInfo::new("AAC");
        // Sample rate
        let sample_rate_index = ((buffer[2] >> 2) & 0x0F) as usize;
        let sample_rates = [
            96000, 88200, 64000, 48000, 44100, 32000, 24000, 22050, 16000, 12000, 11025, 8000, 7350,
            0, 0, 0,
        ];
        info.sample_rate = sample_rates.get(sample_rate_index).copied();

        // Canaux
        let channel_config = ((buffer[2] & 0x01) << 2) | ((buffer[3] >> 6) & 0x03);
        info.channels = Some(channel_config as u16);

        // Taille du fichier
        let file_size = file.metadata().ok()?.len();
        info.file_size = Some(file_size);

        // Codec
        info.codec = Some("AAC (ADTS)".to_string());

        let frame_length = (((buffer[3] & 0x03) as u16) << 11)
            | ((buffer[4] as u16) << 3)
            | ((buffer[5] >> 5) as u16);

        if let Some(sample_rate) = info.sample_rate {
            let frame_duration = 1024.0 / sample_rate as f64;
            let bitrate = ((frame_length as f64 * 8.0) / frame_duration) as u32;
            info.bitrate = Some(bitrate);
            info.duration = Some(file_size as f64 * 8.0 / bitrate as f64);
        }

        // Durée


        return Some(info);
    }

    None
}
