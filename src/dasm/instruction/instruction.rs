use crate::dasm::ByteCodeReader;
use crate::dasm::instruction::{Offset, OPCode, Peek, peek_opcode, operand};

#[derive(Clone, Copy, Debug)]
pub struct Instruction<'a> {
    pub size: usize,
    pub opcode: OPCode,
    pub operand: Option<&'a [u8]>,
}

pub(crate) fn disasm_one<'a>(offset: usize, bytecode: &'a [u8]) -> Option<Offset<Instruction<'a>>> {
    let Peek::<OPCode> { data: opcode, size: opcode_size } = peek_opcode(bytecode)?;
    let operand = operand(opcode, &bytecode[opcode_size..]);

    let instruction = Instruction {
        opcode,
        operand,
        size: opcode_size + operand.map(<[u8]>::len).unwrap_or(0),
    };

    Some(Offset {
        offset,
        data: instruction,
    })
}

pub struct Instructions<'a> {
    reader: ByteCodeReader<'a>,
}

impl<'a> Instructions<'a> {
    pub(crate) fn new(reader: ByteCodeReader<'a>) -> Self {
        Instructions { reader }
    }
}

impl<'a> Iterator for Instructions<'a> {
    type Item = Offset<Instruction<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.next()
    }
}
