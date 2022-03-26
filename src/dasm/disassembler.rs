use crate::dasm::{ByteCodeReader, Instructions};


pub struct Disassembler<'a> {
    reader: ByteCodeReader<'a>,
}

impl<'a> Disassembler<'a> {
    pub fn new(input: &'a[u8]) -> Self {
        let reader = ByteCodeReader::<'a>::new(input);
        Disassembler { reader }
    }

    pub fn disassemble(self) -> Instructions<'a> {
        Instructions::new(self.reader)
    }
}
