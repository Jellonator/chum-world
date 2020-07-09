use std::error::Error;
use std::io::Write;

pub trait ChumExport {
    fn export<W>(&self, writer: &mut W) -> Result<(), Box<dyn Error>>
    where
        W: Write;
}
