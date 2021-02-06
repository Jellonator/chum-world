// use crate::scene;
// use crate::scene::collada::*;
use crate::util::xml::{XMLAttribute, XMLContent, XMLTag, XMLVectorifyTag};

/******************\
|* LIBRARY IMAGES *|
\******************/

#[derive(Debug, Clone)]
pub struct LibraryImages {
    pub image: Vec<Image>,
    pub id: Option<String>,
    pub name: Option<String>,
}

impl_tag_tree!(
    LibraryImages,
    "library_images",
    attr => [("id", id), ("name", name)],
    tags => [image]
);

#[derive(Debug, Clone)]
pub struct Image {
    pub id: Option<String>,
    pub name: Option<String>,
    pub sid: Option<String>,
    // renderable
    pub init_from: ImageInitFrom,
    // create_2d
    // create_3d
    // create_cube
}

impl_tag_tree!(
    Image,
    "image",
    attr => [("id", id), ("name", name), ("sid", sid)],
    tags => [init_from]
);

#[derive(Debug, Clone)]
pub struct ImageInitFrom {
    // mips_generate
    // array_index
    // mip_index
    // depth
    // face
    // ref
    pub file_name: String,
    pub file_ref: Option<ImageInitFromRef>
}

impl ImageInitFrom {
    pub fn get_file_name(&self) -> &str {
        if let Some(fromref) = &self.file_ref {
            &fromref.file_name
        } else {
            &self.file_name
        }
    }
}

impl XMLTag for ImageInitFrom {
    fn get_name(&self) -> &str {
        "init_from"
    }
    fn get_attributes(&self) -> Vec<(&str, &dyn XMLAttribute)> {
        Vec::new()
    }
    fn get_contents(&self) -> Option<&dyn XMLContent> {
        Some(&self.file_name as &dyn XMLContent)
    }
    fn get_child_tags<'a>(&self) -> Vec<&dyn XMLTag> {
        self.file_ref.vectorify()
    }
}

#[derive(Debug, Clone)]
pub struct ImageInitFromRef {
    pub file_name: String
}

impl_tag_content!(ImageInitFromRef, "ref", file_name);

/*******************\
|* LIBRARY EFFECTS *|
\*******************/

#[derive(Debug, Clone)]
pub struct LibraryEffects {
    pub id: Option<String>,
    pub name: Option<String>,
    pub effect: Vec<Effect>
}

#[derive(Debug, Clone)]
pub struct Effect {
    id: String,
    name: Option<String>,
    // newparam
    // profile_COMMON
}

#[derive(Debug, Clone)]
pub struct EffectProfileCommon {
    
}

/*********************\
|* LIBRARY MATERIALS *|
\*********************/

#[derive(Debug, Clone)]
pub struct LibraryMaterials {
    pub material: Vec<Material>,
    pub id: Option<String>,
    pub name: Option<String>,
}

impl_tag_tree!(
    LibraryMaterials,
    "library_materials",
    attr => [("id", id), ("name", name)],
    tags => [material]
);

#[derive(Debug, Clone)]
pub struct Material {
    pub id: Option<String>,
    pub name: Option<String>,
    pub instance_effect: InstanceEffect
}

impl_tag_tree!(
    Material,
    "material",
    attr => [("id", id), ("name", name)],
    tags => [instance_effect]
);

#[derive(Debug, Clone)]
pub struct InstanceEffect {
    pub sid: Option<String>,
    pub name: Option<String>,
    pub url: String,
    pub technique_hint: Vec<TechniqueHint>,
    // pub setparam: Vec<SetParam>
}

impl_tag_tree!(
    InstanceEffect,
    "instance_effect",
    attr => [("sid", sid), ("name", name), ("url", url)],
    tags => [technique_hint]
);

#[derive(Debug, Clone)]
pub struct TechniqueHint {
    pub name: Option<String>,
    pub platform_ref: String,
    pub profile: Option<String>
}

impl_tag_tree!(
    TechniqueHint,
    "technique_hint",
    attr => [("profile", profile), ("name", name), ("ref", platform_ref)],
    tags => []
);

// #[derive(Debug, Clone)]
// pub struct SetParam {
//     pub parameter_ref: String,
// }

// impl_tag_tree!(
//     SetParam,
//     "setparam",
//     attr => [("ref", parameter_ref)],
//     tags => []
// );