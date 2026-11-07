use alloc::vec::Vec;

pub enum JsonParserStack {
    Object {
        placeholder: usize,
    },
    Array {
        placeholder: usize,
        values: Vec<usize>,
    },
}
