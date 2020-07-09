use crate::util;
use libchum::{format::TotemFormat, ChumArchive, ChumFile};
use serde_json;
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Represents the data stored in the .json file.
#[derive(Serialize, Deserialize)]
pub struct JsonData {
    pub header: String,
    pub files: Vec<JsonDataFile>,
    #[serde(default)]
    pub unused_names: Vec<String>
}

impl JsonData {
    pub fn sort(&mut self) {
        self.files.sort_unstable_by(|a, b| a.id.cmp(&b.id));
    }

    pub fn exists(&self, name: &str) -> bool {
        match self.files.binary_search_by_key(&name, |a| &a.id) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

/// Represents a file element in the .json file.
#[derive(Serialize, Deserialize, Clone)]
pub struct JsonDataFile {
    pub id: String,
    pub type_id: String,
    pub subtype_id: String,
    pub file_name: String,
}

/// Get the output file name for the given file string and id
pub fn get_file_string(s: &str, id: u32) -> Vec<String> {
    let fullpath = if let Some(pos) = s.rfind('.') {
        let (left, right) = s.split_at(pos);
        format!("{}[{:08X}]{}", left, id, right)
    } else {
        format!("{}[{:08X}]", s, id)
    };
    let fullpath: &str = if s.starts_with("DB:>") {
        &fullpath[4..]
    } else {
        &fullpath
    };
    fullpath
        .split('>')
        .map(|s| s.replace(|c: char| !c.is_alphanumeric() && c != '.', "_"))
        .collect()
}

/// Extract the given archive into the given output folder.
pub fn extract_archive(
    archive: &ChumArchive,
    output_folder: &Path,
    merge: bool,
) -> Result<(), Box<dyn Error>> {
    let json_path = output_folder.join("meta.json");
    // create folder
    fs::create_dir_all(output_folder)?;
    // create meta.json file
    let mut json_data = JsonData {
        header: String::from_utf8_lossy(archive.get_header().get_legal_notice()).to_string(),
        files: Vec::new(),
        unused_names: archive.find_unused_names().iter().map(|x| x.to_string()).collect()
    };
    // Iterate files
    for file in archive.get_files() {
        // create data file
        let fname = get_file_string(file.get_name_id(), util::hash_name_u32(file.get_name_id()));
        let fpath = output_folder.join(fname.iter().collect::<PathBuf>());
        fs::create_dir_all(fpath.parent().unwrap())?;
        let mut fh = File::create(fpath)?;
        fh.write_all(&file.get_data())?;
        // create entry in meta.json file
        let jsonfile = JsonDataFile {
            id: file.get_name_id().to_string(),
            type_id: file.get_type_id().to_string(),
            subtype_id: file.get_subtype_id().to_string(),
            file_name: fname.join("/"),
        };
        json_data.files.push(jsonfile);
    }
    // sort files
    json_data.sort();
    if merge {
        // attempt to merge files
        if let Ok(prev_file) = File::open(&json_path) {
            let prev_json_data: JsonData = serde_json::from_reader(prev_file)?;
            let mut new_files: Vec<JsonDataFile> = prev_json_data
                .files
                .iter()
                .filter(|file| !json_data.exists(&file.id))
                .map(|file| file.clone())
                .collect();
            json_data.files.append(&mut new_files);
            json_data.sort();
        } else {
            eprintln!("Warning: no existing meta.json file to merge with; skipping merge step.");
        }
    }
    let mut json_file = File::create(&json_path)?;
    serde_json::to_writer_pretty(&mut json_file, &json_data)?;
    Ok(())
}

/// Import an archive from the given path
pub fn import_archive(
    input_folder: &Path,
    fmt: TotemFormat,
) -> Result<ChumArchive, Box<dyn Error>> {
    let meta_path = input_folder.join("meta.json");
    let meta_file = File::open(&meta_path)?;
    let mut json_data: JsonData = serde_json::from_reader(meta_file)?;
    json_data.sort();
    let mut files = Vec::new();
    for file in json_data.files {
        let path: PathBuf = input_folder.join(file.file_name.split('/').collect::<PathBuf>());
        let mut file_handle = File::open(path)?;
        let mut data = Vec::new();
        file_handle.read_to_end(&mut data)?;
        let chumfile = ChumFile::new(data, file.id, file.type_id, file.subtype_id);
        files.push(chumfile);
    }
    Ok(ChumArchive::new_from_files(
        libchum::dgc::TotemHeader::new(&json_data.header.as_bytes()).unwrap(),
        fmt,
        files,
    )?)
}
