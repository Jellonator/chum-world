use std::io::{self, Read, BufRead, BufReader, Write};
use std::error::Error;
use std::collections::HashMap;

/// .NGC archive
/// Contains multiple NGC elements
/// Format (one on each line for each element):
/// <ID> "FILENAME"
/// The ID matches the type_id in DGC data
/// Note that ID is just the crc32::checksum_ieee hash of FILENAME
pub struct NgcArchive {
    names: HashMap<i32, String>,
}

impl NgcArchive {
    /// Create a new Archive file.
    pub fn new(names: HashMap<i32, String>) -> NgcArchive {
        NgcArchive {
            names
        }
    }

    // Get the hashmap of names
    pub fn get_names(&self) -> &HashMap<i32, String> {
        &self.names
    }

    // Claim the names from this archive
    pub fn take_names(self) -> HashMap<i32, String> {
        self.names
    }

    /// Write the archive to the given Writer.
    pub fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        for (id, name) in &self.names {
            writeln!(writer, "{} \"{}\"", id, name)?;
        }
        Ok(())
    }

    /// Read names from the given Reader
    pub fn read_from<R: Read>(reader: &mut R) -> Result<NgcArchive, Box<dyn Error>> {
        let file = BufReader::new(reader);
        
        let mut elements = HashMap::new();

        for line in file.lines() {
            let line = line?;
            if line.len() == 0 || line.starts_with('\0') {
                break;
            }
            let pos = line.find(char::is_whitespace).unwrap();
            let id_str = &line[0..pos];
            let file_str = &line[pos+1..];
            let filelen = file_str.len();
            let id = id_str.parse()?;
            let filename = file_str[1..filelen-1].to_string();
            elements.insert(id, filename);
        }

        Ok(NgcArchive{
            names: elements,
        })
    }
}