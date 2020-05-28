use gdnative::*;

pub mod readerbitmap;
pub mod readertext;
pub mod readertmesh;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct ChumReader;

use crate::chumfile::ChumFile;

#[methods]
impl ChumReader {
    #[export]
    pub fn read_text(&self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| readertext::read_text_from_res(x))
            .unwrap()
    }

    #[export]
    pub fn read_tmesh(&self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| readertmesh::read_tmesh_from_res(x))
            .unwrap()
    }

    #[export]
    pub fn read_bitmap(&self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| readerbitmap::read_bitmap_from_res(x))
            .unwrap()
    }

    #[export]
    pub fn cool(&self, _owner: Node) {
        // very important function do not remove
        godot_print!("Trans Rights!");
    }

    fn _init(_owner: Node) -> Self {
        ChumReader
    }
}
