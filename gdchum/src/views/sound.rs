use crate::chumfile::ChumFile;
use gdnative::api::audio_stream_sample::Format;
use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::reader::sound;
use std::error::Error;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct SoundView {
    pub inner: Option<sound::SoundGcn>,
}

#[methods]
impl SoundView {
    fn new(_owner: &Resource) -> Self {
        SoundView { inner: None }
    }

    fn _register(_builder: &ClassBuilder<Self>) {}

    pub fn set_data(&mut self, data: sound::SoundGcn) {
        self.inner = Some(data);
    }

    #[export]
    pub fn load(&mut self, _owner: &Resource, data: Instance<ChumFile, Shared>) {
        if let Err(e) = self.load_from(data) {
            display_err!("Error while loading SOUND into view: {}", e);
        }
    }

    #[export]
    pub fn save(&self, _owner: &Resource, data: Instance<ChumFile, Shared>) {
        use libchum::binary::ChumBinary;
        if let Some(snd) = self.inner.as_ref() {
            let mut v: Vec<u8> = Vec::new();
            unsafe { data.assume_safe() }
                .map_mut(|chumfile, _| {
                    snd.write_to(&mut v, chumfile.get_format()).unwrap();
                    chumfile.replace_data_with_vec(v);
                })
                .unwrap();
        }
    }

    pub fn load_from(&mut self, data: Instance<ChumFile, Shared>) -> Result<(), Box<dyn Error>> {
        use libchum::binary::ChumBinary;
        unsafe {
            let data = data.assume_safe();
            self.inner = Some(data.map(|cfile, _| {
                cfile.borrow_data(|mut inner_data| {
                    sound::SoundGcn::read_from(&mut inner_data, cfile.get_format())
                })
            })??);
        }
        Ok(())
    }

    #[export]
    pub fn get_format(&self, _owner: &Resource) -> i64 {
        Format::_16_BITS.0
    }

    #[export]
    pub fn get_stream(&self, _owner: &Resource) -> ByteArray {
        let stream = self.inner.as_ref().unwrap().gen_samples();
        let stream_u8 =
            unsafe { std::slice::from_raw_parts(stream.as_ptr() as *const u8, stream.len() * 2) };
        ByteArray::from_slice(stream_u8)
    }

    #[export]
    pub fn is_stereo(&self, _owner: &Resource) -> bool {
        false
    }

    #[export]
    pub fn get_mix_rate(&self, _owner: &Resource) -> i64 {
        self.inner.as_ref().unwrap().sample_rate as i64
    }

    #[export]
    pub fn import_wav(&mut self, _owner: &Resource, path: String) {
        use hound;
        if let Some(snd) = self.inner.as_mut() {
            let mut reader = hound::WavReader::open(path).unwrap();
            snd.sample_rate = reader.spec().sample_rate;
            let samples: Vec<i16> = reader.samples::<i16>().map(|x| x.unwrap()).collect();
            snd.import_samples(samples.as_slice());
        }
    }
}
