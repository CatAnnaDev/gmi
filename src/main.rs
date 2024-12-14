mod formats;
mod audio_info;
mod tags;
mod gmi_error;

use audio_info::AudioInfo;

use std::fs::File;
use std::path::Path;
use std::{fs, io};
use std::io::{BufRead, BufReader, ErrorKind, Seek};
use crate::formats::{aac, flac, mp3, wav, FileFormat};
use crate::formats::FileFormat::*;
use crate::gmi_error::GMIError::*;
use crate::gmi_error::GMIResult;

fn detect_audio_format<R: BufRead + Seek>(reader: &mut R) -> GMIResult<FileFormat> {

    let mut header = [0; 12];
    reader.read_exact(&mut header)?;

    if header.len() < 2 {
        return Err(
            io::Error::new(ErrorKind::UnexpectedEof, "Not enough data").into(),
        );
    }

    if aac::matches(&mut header){
        return Ok(AAC)
    }

    if flac::matches(&mut header){
        return Ok(FLAC)
    }

    if mp3::read_mp3v1_info(&mut header){
        return Ok(MP3v1)
    }

    if mp3::read_mp3v2_info(&mut header){
        return Ok(MP3v2)
    }

    if wav::matches(&mut header){
        return Ok(WAV)
    }

    println!("Format audio non supporté ou fichier invalide.\n");
    Err(NotSupported)
}

fn dispatch_header<R: BufRead + Seek>(reader: &mut R) -> GMIResult<AudioInfo>{
    match detect_audio_format(reader)? {
        MP3v1 => tags::id3v1::read_id3v1_tag(reader),
        MP3v2 => tags::id3v2::read_id3v2_tag(reader),
        WAV => wav::read_wav_info(reader),
        AAC => aac::read_aac_info(reader),
        FLAC => flac::read_flac_info(reader),
    }
}

fn fn_reader<P: AsRef<Path>>(path: P) -> GMIResult<AudioInfo> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    dispatch_header(&mut reader)
}

fn main() {
    let path = fs::read_dir("/Users/anna/Desktop/sound").unwrap();
    for p in path {
        let p = p.unwrap().path();
        println!("{:?}", p.file_name());
        if let Ok(e) = fn_reader(&p) {
            e.display()
        }
    }

}
