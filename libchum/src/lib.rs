pub mod dgc;
pub mod format;
pub mod ngc;
pub mod reader;
use crc::crc32;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::io::Read;

/// Hash the given name using the crc32 IEEE algorithm.
pub fn hash_name(name: &str) -> i32 {
    crc32::checksum_ieee(&name.as_bytes()) as i32
}

/// Complete Chum archive.
/// This is literally all the data that matters.
pub struct ChumArchive {
    header: dgc::TotemHeader,
    files: Vec<ChumFile>,
    names: HashMap<i32, String>,
    format: format::TotemFormat,
}

/// A ChumFile that is returned by the Chum Archive
pub struct ChumFile {
    data: Vec<u8>,
    type_id: String,
    name_id: String,
    subtype_id: String,
}

impl ChumFile {
    /// Create a new ChumFile
    pub fn new(data: Vec<u8>, nameid: String, typeid: String, subtypeid: String) -> ChumFile {
        ChumFile {
            data,
            type_id: typeid,
            name_id: nameid,
            subtype_id: subtypeid,
        }
    }

    /// Get the file's data
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    /// Get the file's name
    pub fn get_name_id(&self) -> &str {
        &self.name_id
    }

    /// Get the file's type
    pub fn get_type_id(&self) -> &str {
        &self.type_id
    }

    /// Get the file's subtype
    pub fn get_subtype_id(&self) -> &str {
        &self.subtype_id
    }
}

#[derive(Debug)]
pub enum ChumError {
    NameMissingError {
        id: i32,
    },
    NameCollisionError {
        id: i32,
        existing_name: String,
        new_name: String,
    },
}

impl Display for ChumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChumError::NameMissingError { id } => {
                write!(f, "Could not find ID {} in NGC archive", id)
            }
            ChumError::NameCollisionError {
                id,
                existing_name,
                new_name,
            } => write!(
                f,
                "Name collision: names {} and {} have colliding ID {}.",
                existing_name, new_name, id
            ),
        }
    }
}

impl Error for ChumError {}

impl ChumArchive {
    /// Create a new, empty chum archive
    pub fn new(header: dgc::TotemHeader, fmt: format::TotemFormat) -> ChumArchive {
        ChumArchive {
            header,
            files: Vec::new(),
            names: HashMap::new(),
            format: fmt,
        }
    }

    /// Check if the given ID can be added.
    /// Returns Err if ID has a hash collision with an existing name
    /// Returns Ok(None) if ID already exists
    /// Returns OK(Some(i32)) if ID needs to be inserted
    fn check_can_add_id(&mut self, s: &str) -> Result<Option<i32>, Box<dyn Error>> {
        let hash = hash_name(s);
        match self.names.get(&hash) {
            Some(existing) => {
                if existing == s {
                    Ok(None)
                } else {
                    Err(Box::new(ChumError::NameCollisionError {
                        id: hash,
                        existing_name: existing.into(),
                        new_name: s.into(),
                    }))
                }
            }
            None => Ok(Some(hash)),
        }
    }

    pub fn add_file(&mut self, file: ChumFile) -> Result<(), Box<dyn Error>> {
        // Check if names can be added
        let hashname = self.check_can_add_id(file.get_name_id())?;
        let hashtype = self.check_can_add_id(file.get_type_id())?;
        let hashsubtype = self.check_can_add_id(file.get_subtype_id())?;
        // Add names if they don't already exist
        if let Some(i) = hashname {
            self.names.insert(i, file.get_name_id().into());
        }
        if let Some(i) = hashtype {
            self.names.insert(i, file.get_type_id().into());
        }
        if let Some(i) = hashsubtype {
            self.names.insert(i, file.get_subtype_id().into());
        }
        // Add file
        self.files.push(file);
        Ok(())
    }

    /// Get this archive's Dgc header
    pub fn get_header(&self) -> &dgc::TotemHeader {
        &self.header
    }

    /// Get all files in this archive
    pub fn get_files(&self) -> impl Iterator<Item = &ChumFile> {
        self.files.iter()
    }

    /// Take all the files from this archive
    pub fn take_files(self) -> impl Iterator<Item = ChumFile> {
        self.files.into_iter()
    }

    /// Split this ChumArchive into an NgcArchive and a DgcArchive
    pub fn split_archives(&self) -> Option<(ngc::TotemNameTable, dgc::TotemArchive)> {
        let dgc = dgc::TotemArchive::new_from_files(
            self.header.clone(),
            self.files
                .iter()
                .map(|file| {
                    dgc::TotemFile::new(
                        file.data.clone(),
                        hash_name(file.get_type_id()),
                        hash_name(file.get_name_id()),
                        hash_name(file.get_subtype_id()),
                    )
                })
                .collect(),
            self.format,
        )?;
        let ngc = ngc::TotemNameTable::new(self.names.clone());
        Some((ngc, dgc))
    }

    /// Merge an NGC and DGC archive
    pub fn merge_archives(
        ngc: ngc::TotemNameTable,
        dgc: dgc::TotemArchive,
    ) -> Result<ChumArchive, Box<dyn Error>> {
        // Check NGC data for matching names
        for file in dgc.iter_files() {
            if !ngc.get_names().contains_key(&file.get_type_id()) {
                return Err(Box::new(ChumError::NameMissingError {
                    id: file.get_type_id(),
                }));
            }
            if !ngc.get_names().contains_key(&file.get_name_id()) {
                return Err(Box::new(ChumError::NameMissingError {
                    id: file.get_name_id(),
                }));
            }
            if !ngc.get_names().contains_key(&file.get_subtype_id()) {
                return Err(Box::new(ChumError::NameMissingError {
                    id: file.get_subtype_id(),
                }));
            }
        }
        let fmt = dgc.get_format();
        // Return archive
        Ok(ChumArchive {
            header: dgc.get_header().clone(),
            files: dgc
                .take_files()
                .into_iter()
                .map(|file| {
                    let type_id = file.get_type_id();
                    let name_id = file.get_name_id();
                    let subtype_id = file.get_subtype_id();
                    ChumFile {
                        data: file.take_data(),
                        type_id: ngc.get_names()[&type_id].clone(),
                        name_id: ngc.get_names()[&name_id].clone(),
                        subtype_id: ngc.get_names()[&subtype_id].clone(),
                    }
                })
                .collect(),
            names: ngc.take_names(),
            format: fmt,
        })
    }

    /// Read the chum archive from two readers
    pub fn read_chum_archive<R: Read, S: Read>(
        ngc_reader: &mut R,
        dgc_reader: &mut S,
        format: format::TotemFormat,
    ) -> Result<ChumArchive, Box<dyn Error>> {
        // Load data
        let ngc = ngc::TotemNameTable::read_from(ngc_reader)?;
        let dgc = dgc::TotemArchive::read_from(dgc_reader, format)?;
        // merge
        ChumArchive::merge_archives(ngc, dgc)
    }

    pub fn get_format(&self) -> format::TotemFormat {
        self.format
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
