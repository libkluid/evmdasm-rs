use crate::analyzer::Context;
use crate::{Offset, Disassembler, Instruction, OPCode};

pub struct CodeAnalyzer<'a> {
    instructions: Vec<Offset<Instruction<'a>>>,
}

impl<'a> CodeAnalyzer<'a> {
    pub fn new(dasm: Disassembler<'a>) -> Self {
        let instructions = dasm.disassemble().collect();

        CodeAnalyzer {
            instructions,
        }
    }

    fn collect_jumptables(&self, context: &mut Context) {
        for ins in self.instructions.iter() {
            match ins.opcode {
                OPCode::JUMPDEST => {
                    context.add_jumpdest(ins.offset, ins);
                }
                OPCode::JUMP | OPCode::JUMPI => {
                    context.add_jump(ins.offset, ins);
                }
                _ => (),
            }
        }
        
        log::debug!("Collected {} jumpdests", context.jumpdest_count());
        log::debug!("Collected {} branches", context.branch_count());
    }

    fn collect_strings(&self, context: &mut Context) {

        let non_control_ascii_filter = |b: &u8| match !b.is_ascii_control() {
            true => Some(*b),
            false => None,
        };

        for ins in self.instructions.iter() {
            match ins.operand {
                None => (),
                Some(bytes) => {
                    if bytes.iter().all(u8::is_ascii) {
                        let non_control_ascii_bytes = bytes.into_iter().filter_map(non_control_ascii_filter).collect();
                        let literal = match String::from_utf8(non_control_ascii_bytes) {
                            Ok(string) if string.len() > 0 => string,
                            _ => continue,
                        };
                        context.add_string(ins.offset, ins, literal);
                    }
                }
            }
        }

        log::debug!("Collected {} strings", context.strings_count());
    }

    pub fn analyze(&self) {
        let mut context = Context::new();
        self.collect_jumptables(&mut context);
        self.collect_strings(&mut context);
    }
}
