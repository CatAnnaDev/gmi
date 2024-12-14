use crate::tags::{id3v1, id3v2};


pub fn read_mp3v1_info(header: &mut [u8]) -> bool {
    id3v1::matches(header)
}

pub fn read_mp3v2_info(header: &mut [u8]) -> bool {
    id3v2::matches(header)
}
