use crate::common::*;

chum_struct_binary! {
    /// Rotation shape
    #[derive(Clone, Default)]
    pub struct RotShape {
        pub header: [struct THeader],
        pub item_type: [ignore [u16] ITEM_TYPE_ROTSHAPE],
        pub item_flags: [u16],
        pub junk1: [ignore [u32] 1],
        pub offset: [Vector3],
        pub junk2: [ignore [u32] 1],
        pub unk7: [reference],
        pub junk3: [ignore [u32] 2],
        pub size: [fixed array [Vector3] 2],
        pub junk4: [ignore [u32] 4],
        pub texcoords: [fixed array [Vector2] 4],
        pub junk5: [ignore [u32] 1],
        pub materialanim_id: [reference MATERIALANIM],
        pub billboard_mode: [enum [u16] BillBoardMode],
    }
}

chum_enum! {
    #[derive(Copy, Clone, Debug)]
    pub enum BillBoardMode {
        YAxis,
        Full,
    }
}

impl Default for BillBoardMode {
    fn default() -> Self {
        BillBoardMode::YAxis
    }
}
