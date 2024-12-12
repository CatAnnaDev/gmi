mod formats;
mod audio_info;
mod tags;

use audio_info::AudioInfo;

use std::fs::File;
use std::path::PathBuf;
use std::{fs, io};
use crate::formats::aac::read_aac_info;
use crate::formats::flac::read_flac_info;
use crate::formats::mp3::read_mp3_info;
use crate::formats::wav::read_wav_info;

fn detect_audio_info(file_path: &PathBuf) -> io::Result<()> {
    let mut file = File::open(file_path)?;

    if let Some(info) = read_mp3_info(&mut file) {
        info.display();
        return Ok(());
    }

    if let Some(info) = read_aac_info(&mut file) {
        info.display();
        return Ok(());
    }

    if let Some(info) = read_wav_info(&mut file) {
        info.display();
        return Ok(());
    }

    if let Some(info) = read_flac_info(&mut file) {
        info.display();
        return Ok(());
    }

    println!("Format audio non supporté ou fichier invalide.");
    Ok(())
}

fn main() {
    let path = fs::read_dir("/Users/anna/Desktop/sound").unwrap();
    for p in path {
        let p = p.unwrap().path();
        println!("{:?}", p.file_name());
        if let Err(e) = detect_audio_info(&p) {
            eprintln!("Erreur lors de la lecture du fichier : {}", e);
        }
    }

}
