use std::collections::BTreeMap;

use crate::{Instruction, OPCode};

pub struct Context {
    offset: usize,
    labels: BTreeMap<usize, String>,
    branches: BTreeMap<usize, Branch>,
    strings: BTreeMap<usize, String>,
}

impl Context {
    pub fn new() -> Self {
        log::debug!("initialize context");
        log::debug!("[{offset:>06X}] JUMPDEST: START", offset=0);

        Context {
            offset: 0_usize,
            labels: [(0, String::from("START"))].into_iter().collect(),
            branches: Default::default(),
            strings: Default::default(),
        }
    }

    pub fn add_jumpdest(&mut self, offset: usize, ins: &Instruction<'_>) {
        log::debug!("[{offset:>06X}] {opcode:?}: LOC_{offset:>06X}", offset=offset, opcode=ins.opcode);
        self.offset = offset;

        let label = format!("LOC_{:>06X}", offset);
        if let Some(prev_label) = self.labels.insert(offset, label) {
            log::error!("[{offset:>06X}] JUMPDEST '{label}' duplicated", offset=offset, label=prev_label)
        }
    }

    pub fn  add_jump(&mut self, offset: usize, ins: &Instruction<'_>) {
        log::debug!("[{offset:>06X}] {opcode:?}", offset=offset, opcode=ins.opcode);
        if let Some(_) = match ins.opcode {
            OPCode::JUMP => self.branches.insert(offset, Branch::Unconditional),
            OPCode::JUMPI => self.branches.insert(offset, Branch::Conditional),
            _ => None
        } {
            log::error!("[{offset:>06X}] {opcode:?} duplicated", offset=offset, opcode=ins.opcode)
        }
    }

    pub fn add_string(&mut self, offset: usize, ins: &Instruction<'_>, s: String) {
        log::debug!("[{offset:>06X}] {opcode:?}: {literal}", offset=offset, opcode=ins.opcode, literal=&s);
        if let Some(ref prev) = self.strings.insert(offset, s) {
            log::error!("[{offset:>06X}] literal '{literal}' duplicated", offset=offset, literal=prev);
        }
    }

    pub fn strings_count(&self) -> usize {
        self.strings.len()
    }

    pub fn branch_count(&self) -> usize {
        self.branches.len()
    }

    pub fn jumpdest_count(&self) -> usize {
        self.labels.len()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Branch {
    Conditional,
    Unconditional,
}
