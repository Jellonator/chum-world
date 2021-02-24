// use crate::common::*;
use crate::util::error::*;
use std::io;
use crate::util;
use crate::format;

const SAMPLES_PER_FRAME: usize = 14;
const BYTES_PER_FRAME: usize = 8;
const NIBBLES_PER_FRAME: usize = 16;
const HEADERS_PER_FRAME: usize = NIBBLES_PER_FRAME - SAMPLES_PER_FRAME;

chum_struct_generate_readwrite! {
    pub struct SoundGcn {
        pub unk0: [u8],
        pub unk0_junk: [ignore [fixed array [u8] 3] [0; 3]],
        pub sample_rate: [u32],
        pub junk: [ignore [fixed array [u8] 4] [0; 4]],
        pub data_length: [u32],
        pub unk1: [ignore [u32] 0u32],
        pub unk2: [ignore [u32] 2u32],
        pub num_adpcm_nibbles: [u32],
        pub unk3: [ignore [u32] 2u32],
        pub unk4: [u32],
        pub unk5: [ignore [u32] 0u32],
        pub unk6: [ignore [u32] 0u32],
        pub unk7: [ignore [i16] 0i16],
        pub coefficients: [fixed array [i16] 16],
        pub unk8: [ignore [i16] 0i16],
        pub first_header: [i16],
        pub unk9: [ignore [i16] 0i16],
        pub unk10: [u32],
        pub unk11: [u32],
        pub data: [custom_binary
            [dynamic array [u32] [u8] 0u8]
            // read number of bytes defined by data_length
            read: |snd: &Inner, file, fmt: format::TotemFormat| -> StructUnpackResult<Vec<u8>> {
                let num_bytes = snd.data_length.unwrap();
                let mut v = vec![0u8; num_bytes as usize];
                match fmt.read_exact(file, v.as_mut_slice()) {
                    Ok(_) => Ok(v),
                    Err(e) => Err(StructUnpackError {
                        structname: "Sound".to_owned(),
                        structpath: "data".to_owned(),
                        error: Box::new(e)
                    })
                }
            };
            // write number of bytes defined by data_length
            write: |value: &Vec<u8>, file, fmt: format::TotemFormat| -> io::Result<()> {
                fmt.write_bytes(file, value.as_slice())
            };
        ]
    }
}

impl SoundGcn {
    // Adapted from https://github.com/Thealexbarney/DspTool
    pub fn gen_samples(&self) -> Vec<i16> {
        let mut hist1 = 0i16;
        let mut hist2 = 0i16;
        let coef = &self.coefficients;
        let frame_count = self.data.len() / BYTES_PER_FRAME;
        let num_samples = self.num_adpcm_nibbles as usize - frame_count * HEADERS_PER_FRAME;
        let mut out = Vec::with_capacity(num_samples);

        for i_frame in 0..frame_count {
            let index = i_frame * BYTES_PER_FRAME;
            let frame = &self.data[index..index+BYTES_PER_FRAME];
            let (h_high, h_low) = util::get_nibbles(frame[0]);
            let predictor: usize = h_high as usize;
            let scale: i32 = 1i32 << h_low as i32;
            let coef1 = coef[predictor * 2];
            let coef2 = coef[predictor * 2 + 1];
            let samples_to_read = SAMPLES_PER_FRAME.min(num_samples - out.len());
            for i_sample in 0..samples_to_read {
                let sample = if i_sample % 2 == 0 {
                    util::get_high_nibble(frame[1 + i_sample/2])
                } else {
                    util::get_low_nibble(frame[1 + i_sample/2])
                } as i32;
                let sample = if sample >= 8 {
                    sample - 16
                } else {
                    sample
                };
                let sample = (((scale * sample) << 11) + 1024 + (coef1 as i32 * hist1 as i32 + coef2 as i32 * hist2 as i32)) >> 11;
                let real_sample = if sample > i16::MAX as i32 {
                    i16::MAX
                } else if sample < i16::MIN as i32 {
                    i16::MIN
                } else {
                    sample as i16
                };
                hist2 = hist1;
                hist1 = real_sample;
                out.push(real_sample);
            }
        }
        out
    }
}
