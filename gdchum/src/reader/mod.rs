use gdnative::*;
use std::collections::HashMap;

pub mod readerbitmap;
pub mod readermaterial;
pub mod readersurface;
pub mod readertext;
pub mod readertmesh;
#[derive(NativeClass)]
#[inherit(Node)]
pub struct ChumReader {
    cache: HashMap<i32, Dictionary>,
}

use crate::chumfile::ChumFile;

#[methods]
impl ChumReader {
    #[export]
    pub fn read_text(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        self.read_text_nodeless(data)
    }
    pub fn read_text_nodeless(&mut self, data: Instance<ChumFile>) -> Dictionary {
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
        self.read_tmesh_nodeless(data)
    }
    pub fn read_tmesh_nodeless(&mut self, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let hash = x.get_name_hash();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = readertmesh::read_tmesh_from_res(x, self);
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_surface(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        self.read_surface_nodeless(data)
    }
    pub fn read_surface_nodeless(&mut self, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let hash = x.get_name_hash();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = readersurface::read_surface_from_res(x, self);
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_bitmap(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        self.read_bitmap_nodeless(data)
    }
    pub fn read_bitmap_nodeless(&mut self, data: Instance<ChumFile>) -> Dictionary {
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
    pub fn read_material(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        self.read_material_nodeless(data)
    }
    pub fn read_material_nodeless(&mut self, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let hash = x.get_name_hash();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = readermaterial::read_material_from_res(x, self);
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
            cache: HashMap::new(),
        }
    }
}
