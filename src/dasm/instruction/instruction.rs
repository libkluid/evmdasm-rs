use crate::dasm::instruction::{OPCode, Peek, peek_opcode, operand};

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
