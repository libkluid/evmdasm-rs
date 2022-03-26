mod peek;
mod opcode;
mod operand;
mod instruction;

pub(self) use peek::Peek;
pub(self) use opcode::peek_opcode;
pub(self) use operand::operand;
pub(in crate::dasm) use instruction::disasm_one;

pub use opcode::OPCode;
pub use instruction::Instruction;
