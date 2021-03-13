//! See https://github.com/Jellonator/chum-world/wiki/CAMERA for more information
use crate::common::*;

// Material data
chum_struct_binary! {
    #[derive(Default, Clone)]
    /// Camera object. Note that fov is in radians.
    pub struct Camera {
        pub header: [struct THeader],
        pub item_type: [ignore [u16] ITEM_TYPE_CAMERA],
        pub item_flags: [u16],
        pub fov: [f32],
        pub unk: [u32],
        pub target: [reference NODE]
    }
}
