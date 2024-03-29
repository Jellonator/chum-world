use crate::chumfile::ChumFile;
use crate::views;
use crate::ChumArchive;
use gdnative::api::{ImageTexture, Resource, ShaderMaterial};
use gdnative::prelude::*;
use libchum::reader;
use std::collections::HashMap;

pub mod bitmap;
pub mod collisionvol;
pub mod lod;
pub mod material;
pub mod materialanim;
pub mod mesh;
pub mod node;
pub mod rotshape;
pub mod skin;
pub mod spline;
pub mod surface;
pub mod text;

pub struct MaterialAnimEntry {
    resource: Ref<ShaderMaterial, Shared>,
    data: reader::materialanim::MaterialAnimation,
    textures: Vec<Option<Ref<Texture, Shared>>>,
    time: f32,
    original_transform: Transform,
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct ChumReader {
    cache: HashMap<i32, Dictionary<Shared>>,
    materialanims: Vec<MaterialAnimEntry>,
}

#[methods]
impl ChumReader {
    #[export]
    pub fn _physics_process(&mut self, _owner: &Node, delta: f32) {
        // godot_print!("{}", delta);
        for entry in &mut self.materialanims {
            if entry.data.length <= 1.0 / 60.0 {
                continue;
            }
            entry.time = (entry.time + delta) % entry.data.length;
            let frame: u16 = (entry.time * 60.0) as u16;

            let material: TRef<ShaderMaterial> = unsafe { entry.resource.assume_safe() };
            if let Some(i) = entry.data.track_texture.find_frame_index(frame) {
                material.set_shader_param("arg_texture", entry.textures[i].clone());
            }
            if let Some((a, b, t)) = entry.data.track_color.find_frame(frame) {
                let color = Vector3::new(
                    a.x * (1.0 - t) + b.x * t,
                    a.y * (1.0 - t) + b.y * t,
                    a.y * (1.0 - t) + b.y * t,
                );
                material.set_shader_param("arg_color", color);
            }
            if let Some((a, b, t)) = entry.data.track_alpha.find_frame(frame) {
                let alpha = a * (1.0 - t) + b * t;
                material.set_shader_param("arg_alpha", alpha);
            }
            if entry.data.track_stretch.len() > 0
                || entry.data.track_scroll.len() > 0
                || entry.data.track_rotation.len() > 0
            {
                let scale = if let Some((a, b, t)) = entry.data.track_stretch.find_frame(frame) {
                    let v = a.lerp(*b, t);
                    Vector2::new(v.x, v.y)
                } else {
                    Vector2::new(1.0, 1.0)
                };
                let scroll = if let Some((a, b, t)) = entry.data.track_scroll.find_frame(frame) {
                    let v = a.lerp(*b, t);
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
                    .then_translate(Vector2::new(-0.5, -0.5))
                    .then_translate(scroll)
                    .then_scale(scale.x, scale.y)
                    .then_rotate(Angle {
                        radians: rotation.into(),
                    })
                    .then_translate(-Vector2::new(0.5, 0.5));
                let realtx = Transform {
                    basis: entry.original_transform.basis
                        * Basis {
                            elements: [
                                Vector3::new(tx.m11, tx.m12, 0.0),
                                Vector3::new(tx.m21, tx.m22, 0.0),
                                Vector3::new(tx.m31, tx.m32, 1.0),
                            ],
                        },
                    origin: entry.original_transform.origin,
                };
                material.set_shader_param("arg_texcoord_transform", realtx);
            }
        }
    }

    pub fn add_materialanim(
        &mut self,
        resource: Ref<ShaderMaterial, Shared>,
        data: reader::materialanim::MaterialAnimation,
        archive: &ChumArchive,
        archive_res: TRef<Resource>,
    ) {
        let mut textures: Vec<Option<Ref<Texture, Shared>>> =
            Vec::with_capacity(data.track_texture.len());
        for track in &data.track_texture.frames {
            let id = track.data;
            textures.push(match archive.get_file_from_hash(&archive_res, id) {
                Some(texturefile) => {
                    let texturedict = self.read_bitmap_nodeless(texturefile.clone());
                    if texturedict.get("exists").to_bool() == true {
                        godot_print!("Found material for {}", id);
                        let image: Ref<Image, Shared> =
                            texturedict.get("bitmap").try_to_object().unwrap();
                        let texture = Ref::<ImageTexture, Unique>::new();
                        texture.create_from_image(image, 2);
                        Some(texture.into_shared().upcast::<Texture>())
                    } else {
                        display_warn!(
                            "Could not apply bitmap {} to materialanim.",
                            unsafe { texturefile.assume_safe() }
                                .map(|x, _| x.get_name_str().to_owned())
                                .unwrap()
                        );
                        None
                    }
                }
                None => {
                    display_warn!("No such bitmap with ID {} to apply to materialanim.", id);
                    None
                }
            });
        }
        // let material: Ref<ShaderMaterial,Shared> = resource.cast().unwrap();
        let tx = unsafe {
            resource
                .assume_safe()
                .get_shader_param("arg_texcoord_transform")
                .to_transform()
        };
        self.materialanims.push(MaterialAnimEntry {
            time: 0.0,
            data,
            textures,
            resource,
            original_transform: tx,
        });
    }

    #[export]
    pub fn read_text(
        &mut self,
        _owner: &Node,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Unique> {
        self.read_text_nodeless(data)
    }
    pub fn read_text_nodeless(&mut self, data: Instance<ChumFile, Shared>) -> Dictionary<Unique> {
        // This is the only file that does not use the cache.
        // This is because any file can be viewed as text, so cacheing it as
        // text can mess up the cache.
        unsafe { data.assume_safe() }
            .map(|x, _| {
                let value = text::read_text_from_res(x);
                value
            })
            .unwrap()
    }

    #[export]
    pub fn read_mesh(
        &mut self,
        _owner: &Node,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        self.read_mesh_nodeless(data)
    }
    pub fn read_mesh_nodeless(&mut self, data: Instance<ChumFile, Shared>) -> Dictionary<Shared> {
        unsafe { data.assume_safe() }
            .map(|x, _| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = mesh::read_mesh_from_res(x, self).into_shared();
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_surface(
        &mut self,
        _owner: &Node,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        self.read_surface_nodeless(data)
    }
    pub fn read_surface_nodeless(
        &mut self,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        unsafe { data.assume_safe() }
            .map(|x, _| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = surface::read_surface_from_res(x, self).into_shared();
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_bitmap(
        &mut self,
        _owner: &Node,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        self.read_bitmap_nodeless(data)
    }
    pub fn read_bitmap_nodeless(&mut self, data: Instance<ChumFile, Shared>) -> Dictionary<Shared> {
        unsafe { data.assume_safe() }
            .map(|x, _| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = bitmap::read_bitmap_from_res(x).into_shared();
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_material(
        &mut self,
        _owner: &Node,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        self.read_material_nodeless(data)
    }
    pub fn read_material_nodeless(
        &mut self,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        unsafe { data.assume_safe() }
            .map(|x, _| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = material::read_material_from_res(x, self).into_shared();
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }
    pub fn read_material_nodeless_nocache(
        &mut self,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Unique> {
        unsafe { data.assume_safe() }
            .map(|x, _| {
                let value = material::read_material_from_res(x, self);
                value
            })
            .unwrap()
    }

    #[export]
    pub fn read_materialanim(
        &mut self,
        _owner: &Node,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        self.read_materialanim_nodeless(data)
    }
    pub fn read_materialanim_nodeless(
        &mut self,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        unsafe { data.assume_safe() }
            .map(|x, _| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = materialanim::read_materialanim_from_res(x, self).into_shared();
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_skin(
        &mut self,
        _owner: &Node,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        self.read_skin_nodeless(data)
    }
    pub fn read_skin_nodeless(&mut self, data: Instance<ChumFile, Shared>) -> Dictionary<Shared> {
        unsafe { data.assume_safe() }
            .map(|x, _| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = skin::read_skin_from_res(x).into_shared();
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_node(
        &mut self,
        _owner: &Node,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        self.read_node_nodeless(data)
    }
    pub fn read_node_nodeless(&mut self, data: Instance<ChumFile, Shared>) -> Dictionary<Shared> {
        unsafe { data.assume_safe() }
            .map(|x, _| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = node::read_node_from_res(x).into_shared();
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_lod(
        &mut self,
        _owner: &Node,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        self.read_lod_nodeless(data)
    }
    pub fn read_lod_nodeless(&mut self, data: Instance<ChumFile, Shared>) -> Dictionary<Shared> {
        unsafe { data.assume_safe() }
            .map(|x, _| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = lod::read_lod_from_res(x).into_shared();
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_rotshape(
        &mut self,
        _owner: &Node,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        self.read_rotshape_nodeless(data)
    }
    pub fn read_rotshape_nodeless(
        &mut self,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        unsafe { data.assume_safe() }
            .map(|x, _| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = rotshape::read_rotshape_from_res(x, self).into_shared();
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_spline(
        &mut self,
        _owner: &Node,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        self.read_spline_nodeless(data)
    }
    pub fn read_spline_nodeless(&mut self, data: Instance<ChumFile, Shared>) -> Dictionary<Shared> {
        unsafe { data.assume_safe() }
            .map(|x, _| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = spline::read_spline_from_res(x).into_shared();
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn read_collisionvol(
        &mut self,
        _owner: &Node,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        self.read_collisionvol_nodeless(data)
    }
    pub fn read_collisionvol_nodeless(
        &mut self,
        data: Instance<ChumFile, Shared>,
    ) -> Dictionary<Shared> {
        unsafe { data.assume_safe() }
            .map(|x, _| {
                let hash = x.get_hash_id_ownerless();
                if let Some(data) = self.cache.get(&hash) {
                    data.new_ref()
                } else {
                    let value = collisionvol::read_collisionvol_from_res(x).into_shared();
                    self.cache.insert(hash, value.new_ref());
                    value
                }
            })
            .unwrap()
    }

    #[export]
    pub fn cool(&self, _owner: &Node) {
        // very important function do not remove
        godot_print!("Trans Rights!");
    }

    #[export]
    pub fn clear_cache(&mut self, _owner: &Node) {
        self.cache.clear();
        self.materialanims.clear();
    }

    #[export]
    pub fn invalidate(&mut self, _owner: &Node, value: i32) {
        self.cache.remove(&value);
    }

    fn new(_owner: &Node) -> Self {
        ChumReader {
            cache: HashMap::new(),
            materialanims: Vec::new(),
        }
    }

    // new View system
    #[export]
    pub fn get_node_view(
        &self,
        _owner: &Node,
        data: Instance<ChumFile, Shared>,
    ) -> Instance<views::node::NodeView, Unique> {
        let instance = Instance::<views::node::NodeView, Unique>::new();
        instance
            .map_mut(|nodeview, _| {
                nodeview.load_from(data).unwrap();
            })
            .unwrap();
        instance
    }

    // new View system
    #[export]
    pub fn get_sound_view(
        &self,
        _owner: &Node,
        data: Instance<ChumFile, Shared>,
    ) -> Instance<views::sound::SoundView, Unique> {
        let instance = Instance::<views::sound::SoundView, Unique>::new();
        instance
            .map_mut(|nodeview, _| {
                nodeview.load_from(data).unwrap();
            })
            .unwrap();
        instance
    }
}
