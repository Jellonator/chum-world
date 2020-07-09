use std::error::Error;
use std::io::Write;

/// Base trait for all structures that can be exported
pub trait ChumExport {
    fn export<W>(&self, writer: &mut W) -> Result<(), Box<dyn Error>>
    where
        W: Write;
}
