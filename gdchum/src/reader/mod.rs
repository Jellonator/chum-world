use crate::ChumArchive;
use gdnative::*;
use libchum::reader;
use std::collections::HashMap;

pub mod bitmap;
pub mod material;
pub mod materialanim;
pub mod skin;
pub mod surface;
pub mod text;
pub mod tmesh;
pub mod node;
pub mod lod;
pub mod rotshape;
pub mod spline;
pub mod collisionvol;

pub struct MaterialAnimEntry {
    resource: Resource,
    data: reader::materialanim::MaterialAnimation,
    textures: Vec<Option<Texture>>,
    time: f32,
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct ChumReader {
    cache: HashMap<i32, Dictionary>,
    materialanims: Vec<MaterialAnimEntry>,
}

use crate::chumfile::ChumFile;

#[methods]
impl ChumReader {
    #[export]
    pub fn _physics_process(&mut self, _owner: Node, delta: f32) {
        // godot_print!("{}", delta);
        for entry in &mut self.materialanims {
            if entry.data.length <= 1.0 / 60.0 {
                continue;
            }
            entry.time = (entry.time + delta) % entry.data.length;
            let frame: u16 = (entry.time * 60.0) as u16;

            let mut material: ShaderMaterial = entry.resource.cast().unwrap();
            if let Some(i) = entry.data.track_texture.find_frame_index(frame) {
                material.set_shader_param("arg_texture".into(), entry.textures[i].to_variant());
            }
            if let Some((a, b, t)) = entry.data.track_color.find_frame(frame) {
                let color = Vector3::new(
                    a[0] * (1.0 - t) + b[0] * t,
                    a[1] * (1.0 - t) + b[1] * t,
                    a[2] * (1.0 - t) + b[2] * t,
                );
                material.set_shader_param("arg_color".into(), color.to_variant());
            }
            if let Some((a, b, t)) = entry.data.track_alpha.find_frame(frame) {
                let alpha = a * (1.0 - t) + b * t;
                material.set_shader_param("arg_alpha".into(), alpha.to_variant());
            }
            if entry.data.track_stretch.len() > 0
                || entry.data.track_scroll.len() > 0
                || entry.data.track_rotation.len() > 0
            {
                let scale = if let Some((a, b, t)) = entry.data.track_stretch.find_frame(frame) {
                    let v = a.lerp(b, t);
                    Vector2::new(v.x, v.y)
                } else {
                    Vector2::new(1.0, 1.0)
                };
                let scroll = if let Some((a, b, t)) = entry.data.track_scroll.find_frame(frame) {
                    let v = a.lerp(b, t);
                    Vector2::new(v.x, v.y)
                } else {
                    Vector2::new(0.0, 0.0)
                };
                let rotation = if let Some((a, b, t)) = entry.data.track_rotation.find_frame(frame)
                {
                    a * (1.0 - t) + b * t
                } else {
                    0.0
                };
                let tx = Transform2D::identity()
                    .post_translate(Vector2::new(-0.5, -0.5))
                    .post_translate(scroll)
                    .post_scale(scale.x, scale.y)
                    .post_rotate(Angle {
                        radians: rotation.into(),
                    })
                    .post_translate(-Vector2::new(0.5, 0.5));
                let realtx = Transform {
                    basis: Basis {
                        elements: [
                            Vector3::new(tx.m11, tx.m12, 0.0),
                            Vector3::new(tx.m21, tx.m22, 0.0),
                            Vector3::new(tx.m31, tx.m32, 1.0),
                        ],
                    },
                    origin: Vector3::new(0.0, 0.0, 0.0),
                };
                material.set_shader_param("arg_texcoord_transform".into(), realtx.to_variant());
            }
        }
    }

    pub fn add_materialanim(
        &mut self,
        resource: Resource,
        data: reader::materialanim::MaterialAnimation,
        archive: &ChumArchive,
        archive_res: Resource,
    ) {
        let mut textures = Vec::with_capacity(data.track_texture.len());
        for track in &data.track_texture.frames {
            let id = track.data;
            textures.push(match archive.get_file_from_hash(archive_res.clone(), id) {
                Some(texturefile) => {
                    let texturedict = self.read_bitmap_nodeless(texturefile);
                    if texturedict.get(&"exists".into()) == true.into() {
                        godot_print!("Found material for {}", id);
                        let image: Image =
                            texturedict.get(&"bitmap".into()).try_to_object().unwrap();
                        let mut texture: ImageTexture = ImageTexture::new();
                        texture.create_from_image(Some(image), 2);
                        Some(texture.cast().unwrap())
                    } else {
                        godot_warn!("Material {} has invalid bitmap", id);
                        None
                    }
                }
                None => None,
            });
        }
        self.materialanims.push(MaterialAnimEntry {
            time: 0.0,
            data,
            textures,
            resource,
        });
    }

    #[export]
    pub fn read_text(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        self.read_text_nodeless(data)
    }
    pub fn read_text_nodeless(&mut self, data: Instance<ChumFile>) -> Dictionary {
        // This is the only file that does not use the cache.
        // This is because any file can be viewed as text, so cacheing it as
        // text can mess up the cache.
        data.script()
            .map(|x| {
                let value = text::read_text_from_res(x);
                value
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
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = tmesh::read_tmesh_from_res(x, self);
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
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = surface::read_surface_from_res(x, self);
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
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = bitmap::read_bitmap_from_res(x);
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
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = material::read_material_from_res(x, self);
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }
    pub fn read_material_nodeless_nocache(&mut self, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let value = material::read_material_from_res(x, self);
                value
            })
            .unwrap()
    }

    #[export]
    pub fn read_materialanim(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        self.read_materialanim_nodeless(data)
    }
    pub fn read_materialanim_nodeless(&mut self, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = materialanim::read_materialanim_from_res(x, self);
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_skin(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        self.read_skin_nodeless(data)
    }
    pub fn read_skin_nodeless(&mut self, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = skin::read_skin_from_res(x);
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_node(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        self.read_node_nodeless(data)
    }
    pub fn read_node_nodeless(&mut self, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = node::read_node_from_res(x);
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_lod(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        self.read_lod_nodeless(data)
    }
    pub fn read_lod_nodeless(&mut self, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = lod::read_lod_from_res(x);
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_rotshape(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        self.read_rotshape_nodeless(data)
    }
    pub fn read_rotshape_nodeless(&mut self, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = rotshape::read_rotshape_from_res(x, self);
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_spline(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        self.read_spline_nodeless(data)
    }
    pub fn read_spline_nodeless(&mut self, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = spline::read_spline_from_res(x);
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_collisionvol(&mut self, _owner: Node, data: Instance<ChumFile>) -> Dictionary {
        self.read_collisionvol_nodeless(data)
    }
    pub fn read_collisionvol_nodeless(&mut self, data: Instance<ChumFile>) -> Dictionary {
        data.script()
            .map(|x| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = collisionvol::read_collisionvol_from_res(x);
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

    #[export]
    pub fn invalidate(&mut self, _owner: Node, value: i32) {
        self.cache.remove(&value);
    }

    fn _init(_owner: Node) -> Self {
        ChumReader {
            cache: HashMap::new(),
            materialanims: Vec::new(),
        }
    }
}
