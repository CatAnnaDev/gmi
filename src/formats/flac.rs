use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::AudioInfo;

pub fn read_flac_info(file: &mut File) -> Option<AudioInfo> {
    let mut buffer = [0u8; 42];
    file.seek(SeekFrom::Start(0)).ok()?;
    file.read_exact(&mut buffer).ok()?;

    if &buffer[0..4] == b"fLaC" {
        let mut info = AudioInfo::new("FLAC");

        let sample_rate = u32::from_be_bytes([buffer[18], buffer[19], buffer[20], 0]) >> 12;
        info.sample_rate = Some(sample_rate);

        let channels = ((buffer[20] & 0x0E) >> 1) + 1;
        info.channels = Some(channels as u16);

        let bit_depth = ((buffer[20] & 0x01) << 4) | ((buffer[21] & 0xF0) >> 4);

        info.bit_depth = Some(bit_depth as u16);


        info.duration = None;

        let file_size = file.metadata().ok()?.len();
        info.file_size = Some(file_size);

        // Codec
        info.codec = Some("FLAC".to_string());

        return Some(info);
    }

    None
}
