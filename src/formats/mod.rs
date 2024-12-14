pub mod mp3;
pub mod wav;
pub mod aac;
pub mod flac;




#[derive(Debug)]
pub enum FileFormat{
    MP3v1,
    MP3v2,
    WAV,
    AAC,
    FLAC,
}