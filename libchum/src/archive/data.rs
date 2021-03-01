use crate::format::TotemFormat;
use std::cmp;
use std::io::{self, Read, Write};
use std::mem;

/// .DGC header information
/// Format:
/// legal notice [u8; 0x100]
/// chunk size   u32         (implied)
/// junk padding [u8; 0x6FC] (ignored)
/// data         [u8; chunk size * N] (N is any whole number)
#[derive(Clone)]
pub struct TotemHeader {
    legal_notice: [u8; 0x100],
}

impl TotemHeader {
    /// Create a new DgcHeader
    /// If the legal notice is larger than 0x0FF, it will be truncated.
    /// Last byte (probably) has to be a null terminator.
    pub fn new(legal: &[u8]) -> TotemHeader {
        let mut headerdata = [0; 0x100];
        for i in 0..legal.len().min(0x0FF) {
            headerdata[i] = legal[i];
        }
        TotemHeader {
            legal_notice: headerdata,
        }
    }

    // Get the legal notice.
    pub fn get_legal_notice(&self) -> &[u8; 0x100] {
        return &self.legal_notice;
    }
}

/// .DGC file element
/// Format:
/// chunk size u32 (implied)
/// type id    i32 (matches the ID in NGC data)
///     This usually refers to some kind of type, e.g. "BITMAP"
/// name_id    i32 (matches the ID in NGC data)
///     This usually refers to some kind of name, e.g. "DB:>LEVELS>LVL_BBEX>LVL_BBEX.TWORLD"
/// subtype_id i32 (matches the ID in NGC data)
///      This is usually either equal to name_id or some kind of subtype, e.g. "LVL_BBEX"
/// data       [u8] (size is chunk size - 16)
pub struct TotemFile {
    data: Vec<u8>,
    type_id: i32,
    name_id: i32,
    subtype_id: i32,
}

impl TotemFile {
    pub fn new(data: Vec<u8>, type_id: i32, name_id: i32, subtype_id: i32) -> TotemFile {
        TotemFile {
            data,
            type_id,
            name_id,
            subtype_id,
        }
    }

    /// Get the type id of this file
    pub fn get_type_id(&self) -> i32 {
        self.type_id
    }

    /// Get the type id of this file
    pub fn get_name_id(&self) -> i32 {
        self.name_id
    }

    /// Get the type id of this file
    pub fn get_subtype_id(&self) -> i32 {
        self.subtype_id
    }

    /// Get the total size of this file, including its header information.
    pub fn get_total_size(&self) -> usize {
        self.data.len() + 16
    }

    /// Get the size of this file's data
    pub fn get_file_size(&self) -> usize {
        self.data.len()
    }

    // Get the file's data
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    // Take the file's data
    pub fn take_data(self) -> Vec<u8> {
        self.data
    }

    /// Write this file to the given writer.
    /// Returns the number of bytes that were written.
    pub fn write_to<W: Write>(&self, writer: &mut W, fmt: TotemFormat) -> io::Result<usize> {
        fmt.write_u32(writer, self.get_total_size() as u32)?;
        fmt.write_i32(writer, self.type_id)?;
        fmt.write_i32(writer, self.name_id)?;
        fmt.write_i32(writer, self.subtype_id)?;
        writer.write_all(&self.data)?;
        Ok(self.get_total_size())
    }
}

/// .DGC chunk
/// Format:
/// num files u32 (implied)
/// data      [u8; chunk size] (inherited from header)
pub struct TotemChunk {
    data: Vec<TotemFile>,
}

impl TotemChunk {
    /// Iterate over all files in this archive.
    pub fn iter_files(&self) -> impl Iterator<Item = &TotemFile> {
        self.data.iter()
    }

    /// Create a new DgcChunk
    pub fn new() -> TotemChunk {
        TotemChunk { data: Vec::new() }
    }

    /// Add a file to this chunk
    fn add_file(&mut self, file: TotemFile) {
        self.data.push(file);
    }

    /// Get the total size of this chunk, including the contents of each file
    /// stored in this chunk, the header data of each file stored in this
    /// chunk, and the header of the chunk itself.
    pub fn get_total_size(&self) -> usize {
        // Each chunk has a 4 byte header, and each file has a 16 byte header
        self.data.iter().fold(4, |acc, f| acc + f.get_total_size())
    }

    /// Get the number of files stored within this chunk.
    pub fn get_num_files(&self) -> usize {
        self.data.len()
    }

    /// Write this chunk to the given writer. Also expects a chunk size argument, that describes
    /// exactly how many bytes this chunk should write. If the chunk is too small to fill this
    /// size, then the chunk will zero-pad the rest.
    /// Returns the number of bytes that were written in total to the writer.
    fn write_to<W: Write>(
        &self,
        writer: &mut W,
        chunk_size: usize,
        fmt: TotemFormat,
    ) -> io::Result<usize> {
        let num_files = self.get_num_files() as u32;
        fmt.write_u32(writer, num_files)?;
        for file in &self.data {
            file.write_to(writer, fmt)?;
        }
        let required_padding = chunk_size - self.get_total_size();
        io::copy(&mut io::repeat(0u8).take(required_padding as u64), writer)?;
        Ok(self.get_total_size() + required_padding)
    }
}

/// .DGC archive
/// Contains the header information about the archive, as well as all of the files stored in the
/// archive sorted into individual chunks. This structure can also serve as an abstraction layer
/// that can automatically divide up files into chunks without having to to worry about the
/// details.
pub struct TotemArchive {
    header: TotemHeader,
    data: Vec<TotemChunk>,
    chunk_size: usize,
    format: TotemFormat,
}

impl TotemArchive {
    /// Get the format
    pub fn get_format(&self) -> TotemFormat {
        self.format
    }

    /// Get the header
    pub fn get_header(&self) -> &TotemHeader {
        &self.header
    }

    /// Get the chunk size
    pub fn get_chunk_size(&self) -> usize {
        self.chunk_size
    }

    /// Create a new DgcArchive. Expects a header and a base chunk size as arguments. The header
    /// must not be bigger than 256 bytes. The base chunk size will be automatically rounded up by
    /// 0x800 bytes, and it may change if files are added to this archive.
    pub fn new(header: TotemHeader, chunk_size: usize, fmt: TotemFormat) -> TotemArchive {
        // let mut headerdata = [0; 0x100];
        // headerdata.copy_from_slice(&header.as_bytes());
        TotemArchive {
            header,
            data: vec![],
            chunk_size: calculate_chunk_size(chunk_size),
            format: fmt,
        }
    }

    /// Create a new DgcArchive with the given filess
    pub fn new_from_files(
        header: TotemHeader,
        files: Vec<TotemFile>,
        fmt: TotemFormat,
    ) -> TotemArchive {
        let mut dgc = TotemArchive::new(header, 0, fmt);
        dgc.set_files(files);
        dgc
    }

    /// Iterate over all files in this archive.
    pub fn iter_files(&self) -> impl Iterator<Item = &TotemFile> {
        self.data.iter().flat_map(|chunk| chunk.data.iter())
    }

    /// Iterate over all chunks in this archive.
    pub fn iter_chunks(&self) -> impl Iterator<Item = &TotemChunk> {
        self.data.iter()
    }

    /// Take all files from this archive
    pub fn take_files(self) -> Vec<TotemFile> {
        self.data
            .into_iter()
            .flat_map(|chunk| chunk.data.into_iter())
            .collect()
    }

    /// Set the files so that the given files all fit
    fn set_files(&mut self, mut files: Vec<TotemFile>) {
        // Clear current chunks
        self.data = Vec::new();
        // Sort files by size. Not the most efficient,
        // but this is the way that DGC files tend to store files.
        files.sort_unstable_by(|a, b| b.data.len().cmp(&a.data.len()));
        // Calculate chunk size
        self.chunk_size = if let Some(f) = files.get(0) {
            calculate_chunk_size(f.get_total_size() + 4)
        } else {
            calculate_chunk_size(0)
        };
        // Pack files.
        // On each iteration, while there is at least one file:
        //     Create a new chunk
        //     Fit as many files as possible in the chunk
        while files.len() > 0 {
            // Create a new chunk
            let mut chunk = TotemChunk::new();
            // Iterate files
            let mut i = 0;
            // Add file if it fits, otherwise skip
            while i < files.len() {
                if files[i].get_total_size() + chunk.get_total_size() <= self.chunk_size {
                    chunk.add_file(files.remove(i));
                } else {
                    i += 1;
                }
            }
            // Push chunk
            self.data.push(chunk);
        }
    }

    /// Add a file to this archive. Will be automatically put into a chunk. This function may
    /// re-distribute files to chunks if the given file is too big to fit into any chunk.
    pub fn add_file(&mut self, file: TotemFile) {
        // If new file doesn't fit, use the set_files function to expand this archive's chunk size
        if file.get_total_size() + 4 > self.chunk_size {
            let mut old_chunks = Vec::new();
            mem::swap(&mut self.data, &mut old_chunks);
            let mut files: Vec<TotemFile> = old_chunks
                .into_iter()
                .flat_map(|chunk| chunk.data.into_iter())
                .collect();
            files.push(file);
            self.set_files(files);
            return;
        }
        // Add the file to the next chunk where it will fit
        for chunk in &mut self.data {
            if chunk.get_total_size() + file.get_total_size() <= self.chunk_size {
                chunk.add_file(file);
                return;
            }
        }
        // Otherwise create a new chunk
        let mut new_chunk = TotemChunk::new();
        new_chunk.add_file(file);
        self.data.push(new_chunk);
    }

    /// Write this archive to a writer.
    pub fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(&self.header.legal_notice)?;
        self.format.write_u32(writer, self.chunk_size as u32)?;
        io::copy(&mut io::repeat(0u8).take(0x6FC), writer)?;
        for chunk in &self.data {
            chunk.write_to(writer, self.chunk_size, self.format)?;
        }
        Ok(())
    }

    /// Create an archive from a reader.
    pub fn read_from<R: Read>(file: &mut R, fmt: TotemFormat) -> io::Result<TotemArchive> {
        let mut legal_notice: [u8; 0x100] = [0; 0x100];
        file.read_exact(&mut legal_notice)?;
        let size = fmt.read_u32(file)?;
        io::copy(&mut file.take(0x6FC), &mut io::sink())?;
        let mut fdata = Vec::new();
        let mut chunks = Vec::new();
        file.read_to_end(&mut fdata)?;
        if fdata.len() % (size as usize) > 0 {
            eprintln!(
                "Warning: stream size {} is not divisible by chunk size {}!",
                fdata.len(),
                size
            );
        }
        for chunk in fdata.chunks(size as usize) {
            chunks.push(load_chunk(chunk, fmt)?);
        }
        Ok(TotemArchive {
            header: TotemHeader {
                legal_notice: legal_notice,
            },
            data: chunks,
            chunk_size: size as usize,
            format: fmt,
        })
    }
}

/// Calculate the size that a chunk would have to be in order to store a file of the given size.
fn calculate_chunk_size(max_size: usize) -> usize {
    // Each chunk's size is a multiple of 0x800 bytes
    const CHUNK_MULT: usize = 0x800;
    if max_size == 0 {
        // avoid subtract with overflow error
        return CHUNK_MULT;
    }
    cmp::max(1, 1 + ((max_size - 1) / CHUNK_MULT)) * CHUNK_MULT
}

/// Load a chunk from the given chunk data.
fn load_chunk(mut data: &[u8], fmt: TotemFormat) -> io::Result<TotemChunk> {
    // let num_files = data.read_u32::<BigEndian>()?;
    let num_files = fmt.read_u32(&mut data)?;
    let mut files = Vec::new();
    for _ in 0..num_files {
        let file_size = fmt.read_u32(&mut data)?;
        let id_type = fmt.read_i32(&mut data)?;
        let id1 = fmt.read_i32(&mut data)?;
        let id2 = fmt.read_i32(&mut data)?;
        let mut contents: Vec<u8> = vec![0; file_size as usize - 16];
        data.read_exact(&mut contents)?;
        files.push(TotemFile {
            data: contents,
            type_id: id_type,
            name_id: id1,
            subtype_id: id2,
        });
    }
    Ok(TotemChunk { data: files })
}
