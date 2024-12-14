use std::io::{BufRead, Seek, SeekFrom};

use crate::AudioInfo;
use crate::gmi_error::{GMIError, GMIResult};

pub fn read_id3v2_tag<R: BufRead + Seek>(reader: &mut R) -> GMIResult<AudioInfo> {
    let mut buffer = [0u8; 10];
    reader.seek(SeekFrom::Start(0)).ok();
    reader.read_exact(&mut buffer).ok();


        let id3_size = ((buffer[6] as usize) << 21)
            | ((buffer[7] as usize) << 14)
            | ((buffer[8] as usize) << 7)
            | (buffer[9] as usize);
        reader.seek(SeekFrom::Start(id3_size as u64)).ok();


        reader.read_exact(&mut buffer).ok();

        loop {
            if buffer[0] == 0xFF && (buffer[1] & 0xE0) == 0xE0 {
                let version = (buffer[1] >> 3) & 0x03;  // Version MPEG (00 = MPEG-1, 01 = MPEG-2, 10 = MPEG-2.5)
                let layer_index = (buffer[1] >> 1) & 0x03;  // Layer (00 = Layer III, 01 = Layer II, 10 = Layer I)
                let bitrate_index = (buffer[2] >> 4) & 0x0F;
                let sample_rate_index = (buffer[2] >> 2) & 0x03;
                let channel_mode = (buffer[3] >> 6) & 0x03;

                let bitrates = [
                    // MPEG-1, MPEG-2, MPEG-2.5 pour chaque Layer
                    // Layer I
                    [
                        [0, 32, 64, 96, 128, 160, 192, 224, 256, 320, 384, 448, 0, 0, 0], // MPEG-1
                        [0, 32, 64, 96, 128, 160, 192, 224, 256, 320, 0, 0, 0, 0, 0], // MPEG-2
                        [0, 32, 64, 96, 128, 160, 192, 224, 256, 320, 0, 0, 0, 0, 0], // MPEG-2.5
                    ],
                    // Layer II
                    [
                        [0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384], // MPEG-1
                        [0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384], // MPEG-2
                        [0, 32, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320, 384], // MPEG-2.5
                    ],
                    // Layer III
                    [
                        [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320], // MPEG-1
                        [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320], // MPEG-2
                        [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320], // MPEG-2.5
                    ]
                ];

                let sample_rates = [44100, 48000, 32000];

                let bitrate = bitrates[layer_index as usize][version as usize -1][bitrate_index as usize] * 1000; // en bps
                let sample_rate = sample_rates[sample_rate_index as usize];

                // Nombre de canaux : mono (1) ou stéréo (2)
                let channels = if channel_mode == 3 { 1 } else { 2 };

                // Informations de codec et durée
                let mut info = AudioInfo::new("MPEG Audio");
                info.bitrate = Some(bitrate);
                info.sample_rate = Some(sample_rate);
                info.channels = Some(channels);



                return Ok(info);
            } else {
                if let Err(e) = reader.read_exact(&mut buffer) {
                    println!("Erreur lors de la lecture des données après ID3: {}", e);
                    return Err(GMIError::CorruptedMediaFile);
                }
            }
        }
}

pub fn matches(header: &[u8]) -> bool{
    header.starts_with(b"ID3")
}
