pub mod bezierpatch;

pub fn round_up(value: usize, mult: usize) -> usize {
    if mult == 0 {
        value
    } else if value % mult == 0 {
        value
    } else {
        value + mult - (value % mult)
    }
}
