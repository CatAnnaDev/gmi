use std::io::{BufRead, Seek, SeekFrom};
use crate::AudioInfo;
use crate::gmi_error::GMIResult;

pub fn read_aac_info<R: BufRead + Seek>(reader: &mut R) -> GMIResult<AudioInfo> {
    let mut buffer = [0u8; 7];
    reader.seek(SeekFrom::Start(0)).ok();
    reader.read_exact(&mut buffer).ok();


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


        // Codec
        info.codec = Some("AAC (ADTS)".to_string());

        let frame_length = (((buffer[3] & 0x03) as u16) << 11)
            | ((buffer[4] as u16) << 3)
            | ((buffer[5] >> 5) as u16);

        if let Some(sample_rate) = info.sample_rate {
            let frame_duration = 1024.0 / sample_rate as f64;
            let bitrate = ((frame_length as f64 * 8.0) / frame_duration) as u32;
            info.bitrate = Some(bitrate); }

        // Durée


        Ok(info)

}


pub fn matches(header: &[u8]) -> bool{
    header.starts_with(b"\xFF\xF1") || header.starts_with(b"\xFF\xF9")
}