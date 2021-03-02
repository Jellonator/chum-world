// use crate::common::*;
use crate::format;
use crate::util::dsp;
use crate::error::*;
use std::io;

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
                        error: e.into()
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
    pub fn gen_samples(&self) -> Vec<i16> {
        let frame_count = self.data.len() / dsp::BYTES_PER_FRAME;
        let num_samples = self.num_adpcm_nibbles as usize - frame_count * dsp::HEADERS_PER_FRAME;
        dsp::decode(&self.coefficients, &self.data, num_samples)
    }

    pub fn import_samples(&mut self, data: &[i16]) {
        let result = dsp::encode(data);
        self.data_length = result.data.len() as u32;
        self.data = result.data;
        self.coefficients = result.coef;
        // let num_frames = util::div_up(self.data_length, dsp::BYTES_PER_FRAME as u32);
        self.num_adpcm_nibbles = self.data_length * 2; // - num_frames * dsp::HEADERS_PER_FRAME as u32;
    }
}
