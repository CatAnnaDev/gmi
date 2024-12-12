use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use crate::AudioInfo;

pub fn read_id3v1_tag(file: &mut File) -> Option<AudioInfo> {
    let mut buffer = [0u8; 128];
    file.seek(SeekFrom::End(-128)).ok()?;
    file.read_exact(&mut buffer).ok()?;

    if &buffer[0..3] == b"TAG" {
        let mut info = AudioInfo::new("MP3 (ID3v1)");

        info.sample_rate = None;
        info.bitrate = None;
        info.channels = None;

        return Some(info);
    }

    None
}
