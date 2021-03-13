use crate::reader::node;

const T_NODE: i32 = -1276508687;

chum_struct_binary! {
    #[derive(Clone, Default)]
    pub struct GameObj {
        pub prefabs: [dynamic array [u32] [struct Prefab] Prefab::default()]
    }
}

chum_struct_binary! {
    #[derive(Clone)]
    pub struct Prefab {
        pub asset_type: [ignore [i32] T_NODE],
        pub subtype1: [reference],
        pub subtype2: [custom_structure [reference]
            // Always same as subtype1 so value does not matter
            structure: |_prefab: &Prefab| {
                None
            };
            // The value of `item_subtype` depends on the presence of the `sounds` value
            destructure: |data: &crate::structure::ChumStructVariant| {
                data.get_struct_item("subtype1").unwrap().get_reference_id().unwrap()
            };
        ],
        pub node: [struct node::Node]
    }
}

impl Default for Prefab {
    fn default() -> Prefab {
        Prefab {
            asset_type: (),
            subtype1: 0,
            subtype2: 0,
            node: node::Node::default(),
        }
    }
}
