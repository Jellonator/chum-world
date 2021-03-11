//! See https://github.com/Jellonator/chum-world/wiki/MATERIALANIM for more information

use crate::common::*;

// /// Interpolation method
// #[derive(Clone, Copy)]
// pub enum Interpolation {
//     /// Discrete interpolation (1)
//     Discrete,
//     /// Linear interpolation (2)
//     Linear,
//     /// Unknown interpolation (3)
//     Unknown,
//     /// Invalid interpolation (error)
//     Invalid,
// }

chum_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Interpolation {
        Invalid,
        Discrete,
        Linear,
        Unknown
    }
}

impl Default for Interpolation {
    fn default() -> Self {
        Interpolation::Discrete
    }
}

/// A single frame in a track
#[derive(Clone, Default)]
pub struct TrackFrame<T>
    where T: Clone + Default
{
    pub frame: u16,
    pub junk: (),
    pub data: T,
}

// Implement ChumBinary for all TrackFrame types
chum_struct_binary_impl! {
    impl ChumBinary for TrackFrame<i32> {
        // IMPORTANT: bitmap_id comes BEFORE frame for TextureFrame.
        data: [i32],
        // TextureFrame is also the only Track with no junk data.
        junk: [ignore [void] ()],
        frame: [u16],
    }
}
chum_struct_binary_impl! {
    impl ChumBinary for TrackFrame<Vector2> {
        frame: [u16],
        junk: [ignore [u16] 0u16],
        data: [Vector2]
    }
}
chum_struct_binary_impl! {
    impl ChumBinary for TrackFrame<f32> {
        frame: [u16],
        junk: [ignore [u16] 0u16],
        data: [f32]
    }
}
chum_struct_binary_impl! {
    impl ChumBinary for TrackFrame<Vector3> {
        frame: [u16],
        junk: [ignore [u16] 0u16],
        data: [Vector3 rgb]
    }
}
chum_struct_binary_impl! {
    impl ChumBinary for TrackFrame<[u8; 4]> {
        frame: [u16],
        junk: [ignore [u16] 0u16],
        data: [fixed array [u8] 4]
    }
}

/// A full track, including a list of frames and interpolation method
#[derive(Clone, Default)]
pub struct Track<T>
    where T: Clone + Default
{
    pub interp: Interpolation,
    pub frames: Vec<TrackFrame<T>>,
}

impl<T> Track<T>
    where T: Clone + Default
{
    /// Find the frame value that the given frame refers to.
    /// Most of the time, the frame index will be between two frames.
    /// Because of this, this function returns the values of the frame before
    /// and the frame after, as well as a float for interpolation.
    pub fn find_frame(&self, frame: u16) -> Option<(&T, &T, f32)> {
        if self.frames.len() == 0 {
            None
        } else if self.frames.len() == 1 {
            Some((&self.frames[0].data, &self.frames[1].data, 0.0))
        } else {
            // Todo: Handle interpolation
            for i in 1..self.frames.len() {
                if self.frames[i].frame >= frame {
                    let prev = &self.frames[i - 1];
                    let curr = &self.frames[i];
                    let t = (frame as f32 - prev.frame as f32)
                        / (curr.frame as f32 - prev.frame as f32);
                    return Some((&prev.data, &curr.data, t));
                }
            }
            Some((
                // unwrap here is fine because we already know that frames.len() > 0
                &self.frames.last().unwrap().data,
                &self.frames.last().unwrap().data,
                0.0,
            ))
        }
    }

    /// Find the index for the frame at or before the given frame.
    pub fn find_frame_index(&self, frame: u16) -> Option<usize> {
        if self.frames.len() == 0 {
            None
        } else if self.frames.len() == 1 {
            Some(0)
        } else {
            // Todo: Handle interpolation
            for i in 1..self.frames.len() {
                if self.frames[i].frame >= frame {
                    return Some(i - 1);
                }
            }
            Some(self.frames.len() - 1)
        }
    }

    /// Get the number of frames in this track
    pub fn len(&self) -> usize {
        self.frames.len()
    }
}

// Implement ChumBinary for all Track types
chum_struct_binary_impl! {
    impl ChumBinary for Track<i32> {
        interp: [enum [u16] Interpolation],
        frames: [dynamic array [u32] [struct TrackFrame<i32>] TrackFrame::<i32>::default()]
    }
}
chum_struct_binary_impl! {
    impl ChumBinary for Track<Vector2> {
        interp: [enum [u16] Interpolation],
        frames: [dynamic array [u32] [struct TrackFrame<Vector2>] TrackFrame::<Vector2>::default()]
    }
}
chum_struct_binary_impl! {
    impl ChumBinary for Track<f32> {
        interp: [enum [u16] Interpolation],
        frames: [dynamic array [u32] [struct TrackFrame<f32>] TrackFrame::<f32>::default()]
    }
}
chum_struct_binary_impl! {
    impl ChumBinary for Track<Vector3> {
        interp: [enum [u16] Interpolation],
        frames: [dynamic array [u32] [struct TrackFrame<Vector3>] TrackFrame::<Vector3>::default()]
    }
}
chum_struct_binary_impl! {
    impl ChumBinary for Track<[u8; 4]> {
        interp: [enum [u16] Interpolation],
        frames: [dynamic array [u32] [struct TrackFrame<[u8; 4]>] TrackFrame::<[u8; 4]>::default()]
    }
}

chum_struct_binary! {
    /// Material animation file
    #[derive(Clone, Default)]
    pub struct MaterialAnimation {
        pub unk1: [u8],
        pub length: [f32],
        pub track_texture: [struct Track<i32>],
        pub track_scroll: [struct Track<Vector2>],
        pub track_stretch: [struct Track<Vector2>],
        pub track_rotation: [struct Track<f32>],
        pub track_color: [struct Track<Vector3>],
        pub track_emission: [struct Track<Vector3>],
        pub track_alpha: [struct Track<f32>],
        pub track_unk1: [struct Track<[u8; 4]>],
        pub track_unk2: [struct Track<[u8; 4]>],
        pub track_unk3: [struct Track<[u8; 4]>],
        pub material_id: [reference MATERIAL],
    }
}

pub const TRACK_TEXTURE: usize = 0;
pub const TRACK_SCROLL: usize = 1;
pub const TRACK_STRETCH: usize = 2;
pub const TRACK_ROTATION: usize = 3;
pub const TRACK_COLOR: usize = 4;
pub const TRACK_EMISSION: usize = 5;
pub const TRACK_ALPHA: usize = 6;
pub const TRACK_UNK1: usize = 7;
pub const TRACK_UNK2: usize = 8;
pub const TRACK_UNK3: usize = 9;

impl MaterialAnimation {
    pub fn get_track_interpolation(&self, index: usize) -> Option<Interpolation> {
        Some(match index {
            0 => self.track_texture.interp,
            1 => self.track_scroll.interp,
            2 => self.track_stretch.interp,
            3 => self.track_rotation.interp,
            4 => self.track_color.interp,
            5 => self.track_emission.interp,
            6 => self.track_alpha.interp,
            7 => self.track_unk1.interp,
            8 => self.track_unk2.interp,
            9 => self.track_unk3.interp,
            _ => None?
        })
    }

    pub fn set_track_interpolation(&mut self, index: usize, value: Interpolation) {
        match index {
            0 => self.track_texture.interp = value,
            1 => self.track_scroll.interp = value,
            2 => self.track_stretch.interp = value,
            3 => self.track_rotation.interp = value,
            4 => self.track_color.interp = value,
            5 => self.track_emission.interp = value,
            6 => self.track_alpha.interp = value,
            7 => self.track_unk1.interp = value,
            8 => self.track_unk2.interp = value,
            9 => self.track_unk3.interp = value,
            _ => {}
        }
    }
}
