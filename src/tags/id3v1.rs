use std::io::{BufRead, Seek, SeekFrom};

use crate::AudioInfo;
use crate::gmi_error::GMIResult;

pub fn read_id3v1_tag<R: BufRead + Seek>(reader: &mut R) -> GMIResult<AudioInfo> {
    let mut buffer = [0u8; 128];
    reader.seek(SeekFrom::End(-128)).ok();
    reader.read_exact(&mut buffer).ok();

        let mut info = AudioInfo::new("MP3 (ID3v1)");

        info.sample_rate = None;
        info.bitrate = None;
        info.channels = None;

        Ok(info)
}

pub fn matches(header: &[u8]) -> bool{
    header.starts_with(b"TAG")
}