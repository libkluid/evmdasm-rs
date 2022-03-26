use crate::dasm::instruction::disasm_one;

use crate::dasm::{Instruction, Offset};

#[derive(Default)]
struct State {
    offset: usize,
}

pub struct ByteCodeReader<'a> {
    bytecode: &'a [u8],
    state: State,
}

impl<'a> ByteCodeReader<'a> {
    pub fn new(bytecode: &'a [u8]) -> Self {
        ByteCodeReader {
            bytecode,
            state: Default::default(),
        }
    }

    fn advance(&mut self, step: usize) {
        self.state.offset += step;
    }

    pub fn next_instrunction(&mut self) -> Option<Offset<Instruction<'a>>> {
        let bytecode = &self.bytecode[(self.state.offset)..];
        let instruction = disasm_one(self.state.offset, bytecode)?;

        self.advance(instruction.data.size);
        Some(instruction)
    }
}

impl<'a> Iterator for ByteCodeReader<'a> {
    type Item = Offset<Instruction<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_instrunction()
    }
}
