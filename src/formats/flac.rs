use std::io::{BufRead, Seek, SeekFrom};

use crate::AudioInfo;
use crate::gmi_error::GMIResult;

pub fn read_flac_info<R: BufRead + Seek>(reader: &mut R) -> GMIResult<AudioInfo> {
    let mut buffer = [0u8; 42];
    reader.seek(SeekFrom::Start(0)).ok();
    reader.read_exact(&mut buffer).ok();

        let mut info = AudioInfo::new("FLAC");

        let sample_rate = u32::from_be_bytes([buffer[18], buffer[19], buffer[20], 0]) >> 12;
        info.sample_rate = Some(sample_rate);

        let channels = ((buffer[20] & 0x0E) >> 1) + 1;
        info.channels = Some(channels as u16);

        let bit_depth = ((buffer[20] & 0x01) << 4) | ((buffer[21] & 0xF0) >> 4);

        info.bit_depth = Some(bit_depth as u16);


        // Codec
        info.codec = Some("FLAC".to_string());

        Ok(info)
}

pub fn matches(header: &[u8]) -> bool{
    header.starts_with(b"fLaC")
}