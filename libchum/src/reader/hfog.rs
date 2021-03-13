use crate::common::*;

chum_struct_binary! {
    #[derive(Clone, Default)]
    pub struct HFog {
        pub header: [struct THeader],
        pub item_type: [ignore [u16] ITEM_TYPE_HFOG],
        pub item_flags: [u16],
        pub color: [Vector3 rgb],
        pub unk0: [ignore [u8] 1],
        pub unk0_junk: [ignore [fixed array [u8] 3] [0u8; 3]],
        pub translation: [Vector3],
        pub scale: [Vector3],
        pub rotation: [Quaternion],
        pub unk5: [Transform3D],
        pub unk6: [Transform3D],
    }
}
