use gdnative::*;
use std::collections::HashMap;
use libchum::reader::materialanim;

pub mod readerbitmap;
pub mod readermaterial;
pub mod readersurface;
pub mod readertext;
pub mod readertmesh;
pub mod readermaterialanim;

pub struct MaterialAnimEntry {
    resource: Resource,
    data: materialanim::MaterialAnimation,
    time: f32
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct ChumReader {
    cache: HashMap<i32, Dictionary>,
    materialanims: Vec<MaterialAnimEntry>
}

use crate::chumfile::ChumFile;

#[methods]
impl ChumReader {
    pub fn add_materialanim(&mut self, resource: Resource, data: materialanim::MaterialAnimation) {
        self.materialanims.push(MaterialAnimEntry {
            time: 0.0,
            data,
            resource
        });
    }

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
    pub fn read_material_nodeless_nocache(&mut self, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let value = readermaterial::read_material_from_res(x, self);
                value
            })
            .unwrap()
    }

    #[export]
    pub fn read_materialanim(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        self.read_material_nodeless(data)
    }
    pub fn read_materialanim_nodeless(&mut self, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let hash = x.get_name_hash();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = readermaterialanim::read_materialanim_from_res(x, self);
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
        self.materialanims.clear();
    }

    fn _init(_owner: Node) -> Self {
        ChumReader {
            cache: HashMap::new(),
            materialanims: Vec::new()
        }
    }
}
