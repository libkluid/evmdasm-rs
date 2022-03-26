use crate::dasm::{instruction::{OPCode, Peek, peek_opcode, operand}, ByteCodeReader};

#[derive(Clone, Copy, Debug)]
pub struct Instruction<'a> {
    pub offset: usize,
    pub size: usize,
    pub opcode: OPCode,
    pub operand: Option<&'a [u8]>,
}

pub(crate) fn disasm_one<'a>(offset: usize, bytecode: &'a [u8]) -> Option<Instruction<'a>> {
    let Peek::<OPCode> { data: opcode, size: opcode_size } = peek_opcode(bytecode)?;
    let operand = operand(opcode, &bytecode[opcode_size..]);

    Some(Instruction { offset, opcode, operand, size: opcode_size + operand.map(<[u8]>::len).unwrap_or(0) })
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
    type Item = Instruction<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.next()
    }
}
