use crate::util;
use gdnative::api::audio_stream_sample::Format;
use gdnative::api::Resource;
use gdnative::prelude::*;
use libchum::reader::sound;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct SoundView {
    pub inner: sound::SoundGcn,
}

#[methods]
impl SoundView {
    fn new(_owner: &Resource) -> Self {
        SoundView {
            inner: Default::default(),
        }
    }

    impl_view!(
        SoundView,
        sound::SoundGcn,
        "SOUND",
        |builder: &ClassBuilder<Self>| {
            builder
                .add_property("mix_rate")
                .with_getter(Self::get_mix_rate)
                .with_setter(Self::set_mix_rate)
                .done();
        }
    );

    #[export]
    pub fn get_format(&self, _owner: &Resource) -> i64 {
        Format::_16_BITS.0
    }

    #[export]
    pub fn get_stream(&self, _owner: &Resource) -> ByteArray {
        let stream = self.inner.gen_samples();
        let stream_u8 =
            unsafe { std::slice::from_raw_parts(stream.as_ptr() as *const u8, stream.len() * 2) };
        ByteArray::from_slice(stream_u8)
    }

    #[export]
    pub fn is_stereo(&self, _owner: &Resource) -> bool {
        false
    }

    #[export]
    pub fn get_mix_rate(&self, _owner: TRef<Resource>) -> i64 {
        self.inner.sample_rate as i64
    }

    #[export]
    pub fn set_mix_rate(&mut self, owner: TRef<Resource>, value: i64) {
        self.inner.sample_rate = value as u32;
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn import_wav(&mut self, owner: &Resource, path: String) {
        use hound;
        let mut reader = hound::WavReader::open(path).unwrap();
        self.inner.sample_rate = reader.spec().sample_rate;
        let samples: Vec<i16> = reader.samples::<i16>().map(|x| x.unwrap()).collect();
        self.inner.import_samples(samples.as_slice());
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_structure(&self, _owner: &Resource) -> Variant {
        use libchum::structure::ChumStruct;
        let data = self.inner.get_struct().structure();
        util::struct_to_dict(&data).into_shared().to_variant()
    }

    #[export]
    pub fn import_structure(&mut self, owner: &Resource, data: Dictionary) {
        use libchum::structure::ChumStruct;
        let structure = util::dict_to_struct(&data);
        self.inner
            .import_struct(&sound::SoundGcnStruct::destructure(&structure).unwrap());
        owner.emit_signal("modified", &[]);
    }
}
