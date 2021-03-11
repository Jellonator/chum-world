use libchum::reader::materialanim::*;
use gdnative::prelude::*;
use gdnative::api::Resource;
use crate::util;

#[derive(NativeClass)]
#[inherit(Resource)]
#[register_with(Self::_register)]
pub struct MaterialAnimView {
    pub inner: MaterialAnimation,
}

fn read_track<T>(track: &Track<T>) -> VariantArray<Unique>
    where T: Clone + Default + ToVariant
{
    let arr = VariantArray::new();
    for value in track.frames.iter() {
        let data = Dictionary::new();
        data.insert("frame", value.frame);
        data.insert("data", value.data.clone());
    }
    arr
}

fn read_track_unk(track: &Track<[u8; 4]>) -> VariantArray<Unique> {
    let arr = VariantArray::new();
    for value in track.frames.iter() {
        let dict = Dictionary::new();
        dict.insert("frame", value.frame);
        let data = ByteArray::from_slice(&value.data);
        dict.insert("data", data);
    }
    arr
}

fn gen_track<T>(data: VariantArray<ThreadLocal>, interp: Interpolation) -> Option<Track<T>>
    where T: Clone + Default + FromVariant
{
    let mut track = Track::<T>::default();
    track.interp = interp;
    for item in data.iter() {
        let dict = Dictionary::from_variant(&item).ok()?;
        let frame = u16::from_variant(&dict.get("frame")).ok()?;
        let value = T::from_variant(&dict.get("data")).ok()?;
        track.frames.push(TrackFrame {
            frame,
            junk: (),
            data: value
        });
    }
    Some(track)
}

fn gen_track_unk(data: VariantArray<ThreadLocal>, interp: Interpolation) -> Option<Track<[u8; 4]>> {
    let mut track = Track::<[u8; 4]>::default();
    track.interp = interp;
    for item in data.iter() {
        let dict = Dictionary::from_variant(&item).ok()?;
        let frame = u16::from_variant(&dict.get("frame")).ok()?;
        let arr = ByteArray::from_variant(&dict.get("data")).ok()?;
        if arr.len() != 4 {
            return None;
        }
        track.frames.push(TrackFrame {
            frame,
            junk: (),
            data: [
                arr.get(0),
                arr.get(1),
                arr.get(2),
                arr.get(3),
            ]
        });
    }
    Some(track)
}

#[methods]
impl MaterialAnimView {
    fn new(_owner: &Resource) -> Self {
        MaterialAnimView { inner:  Default::default() }
    }

    impl_view!(MaterialAnimView, MaterialAnimation, "MATERIALANIM",
        |builder: &ClassBuilder<Self>| {
            builder
                .add_property("material")
                .with_getter(Self::get_material)
                .with_setter(Self::set_material)
                .done();
            builder
                .add_property("length")
                .with_getter(Self::get_length)
                .with_setter(Self::set_length)
                .done();
        }
    );

    #[export]
    pub fn get_structure(&self, _owner: &Resource) -> Variant {
        use libchum::structure::ChumStruct;
        let data = self.inner.structure();
        util::struct_to_dict(&data).into_shared().to_variant()
    }

    #[export]
    pub fn import_structure(&mut self, owner: &Resource, data: Dictionary) {
        use libchum::structure::ChumStruct;
        let structure = util::dict_to_struct(&data);
        self.inner = MaterialAnimation::destructure(&structure).unwrap();
        owner.emit_signal("modified", &[]);
    }
    
    #[export]
    pub fn get_length(&self, _owner: TRef<Resource>) -> f32 {
        self.inner.length
    }

    #[export]
    pub fn set_length(&mut self, owner: TRef<Resource>, value: f32) {
        self.inner.length = value;
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_material(&self, _owner: TRef<Resource>) -> i32 {
        self.inner.material_id
    }

    #[export]
    pub fn set_material(&mut self, owner: TRef<Resource>, value: i32) {
        self.inner.material_id = value;
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_track_interpolation(&self, _owner: TRef<Resource>, index: usize) -> Option<u32> {
        use libchum::structure::ChumEnum;
        self.inner.get_track_interpolation(index).map(|v| v.to_u32())
    }

    #[export]
    pub fn set_track_interpolation(&mut self, owner: TRef<Resource>, index: usize, value: u32) {
        use libchum::structure::ChumEnum;
        self.inner.set_track_interpolation(index, Interpolation::from_u32(value).unwrap());
        owner.emit_signal("modified", &[]);
    }

    #[export]
    pub fn get_track(&self, _owner: TRef<Resource>, index: usize) -> Option<VariantArray> {
        Some(match index {
            TRACK_TEXTURE => read_track(&self.inner.track_texture),
            TRACK_SCROLL => read_track(&self.inner.track_scroll),
            TRACK_STRETCH => read_track(&self.inner.track_stretch),
            TRACK_ROTATION => read_track(&self.inner.track_rotation),
            TRACK_COLOR => read_track(&self.inner.track_color),
            TRACK_EMISSION => read_track(&self.inner.track_emission),
            TRACK_ALPHA => read_track(&self.inner.track_alpha),
            TRACK_UNK1 => read_track_unk(&self.inner.track_unk1),
            TRACK_UNK2 => read_track_unk(&self.inner.track_unk2),
            TRACK_UNK3 => read_track_unk(&self.inner.track_unk3),
            _ => None?
        }.into_shared())
    }

    fn set_track_priv(&mut self, index: usize, data: VariantArray<ThreadLocal>, interp: Interpolation) -> Option<()>
    {
        match index {
            TRACK_TEXTURE => self.inner.track_texture = gen_track(data, interp)?,
            TRACK_SCROLL => self.inner.track_scroll = gen_track(data, interp)?,
            TRACK_STRETCH => self.inner.track_stretch = gen_track(data, interp)?,
            TRACK_ROTATION => self.inner.track_rotation = gen_track(data, interp)?,
            TRACK_COLOR => self.inner.track_color = gen_track(data, interp)?,
            TRACK_EMISSION => self.inner.track_emission = gen_track(data, interp)?,
            TRACK_ALPHA => self.inner.track_alpha = gen_track(data, interp)?,
            TRACK_UNK1 => self.inner.track_unk1 = gen_track_unk(data, interp)?,
            TRACK_UNK2 => self.inner.track_unk2 = gen_track_unk(data, interp)?,
            TRACK_UNK3 => self.inner.track_unk3 = gen_track_unk(data, interp)?,
            _ => None?
        }
        Some(())
    }

    #[export]
    pub fn set_track(&mut self, owner: TRef<Resource>, index: usize, data: VariantArray<Shared>) {
        let data = unsafe { data.assume_unique() }.into_thread_local();
        if let Some(interp) = self.inner.get_track_interpolation(index) {
            match self.set_track_priv(index, data, interp) {
                Some(()) => {
                    owner.emit_signal("modified", &[]);
                },
                None => {
                    display_err!("Could not parse track data");
                }
            }
        } else {
            display_err!("Invalid track {}", index);
        }
    }
}
