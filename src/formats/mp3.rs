use std::fs::File;

use crate::tags::{id3v1::read_id3v1_tag, id3v2::read_id3v2_tag};
use crate::AudioInfo;

pub fn read_mp3_info(file: &mut File) -> Option<AudioInfo> {
    // Vérifie pour un tag ID3v2
    if let Some(info) = read_id3v2_tag(file) {
        return Some(info);
    }

    // Vérifie pour un tag ID3v1
    if let Some(info) = read_id3v1_tag(file) {
        return Some(info);
    }

    None
}
