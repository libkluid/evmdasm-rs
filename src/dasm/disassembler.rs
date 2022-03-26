use crate::dasm::{ByteCodeReader, Instruction};


pub struct Disassembler<'a> {
    reader: ByteCodeReader<'a>,
}

impl<'a> Disassembler<'a> {
    pub fn new(input: &'a[u8]) -> Self {
        let reader = ByteCodeReader::<'a>::new(input);
        Disassembler { reader }
    }

    pub fn disassemble<F>(self) -> F
    where
        F: FromIterator<Instruction<'a>>
    {
        self.reader.collect()
    }
}
