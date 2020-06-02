use gdnative::*;
use std::collections::HashMap;

pub mod readerbitmap;
pub mod readertext;
pub mod readertmesh;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct ChumReader {
    cache: HashMap<i32, Dictionary>
}

use crate::chumfile::ChumFile;

#[methods]
impl ChumReader {
    #[export]
    pub fn read_text(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let hash = x.get_name_hash();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = readertext::read_text_from_res(x);
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_tmesh(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let hash = x.get_name_hash();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = readertmesh::read_tmesh_from_res(x);
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_bitmap(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let hash = x.get_name_hash();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = readerbitmap::read_bitmap_from_res(x);
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn cool(&self, _owner: Node) {
        // very important function do not remove
        godot_print!("Trans Rights!");
    }

    #[export]
    pub fn clear_cache(&mut self, _owner: Node) {
        self.cache.clear();
    }

    fn _init(_owner: Node) -> Self {
        ChumReader {
            cache: HashMap::new()
        }
    }
}
