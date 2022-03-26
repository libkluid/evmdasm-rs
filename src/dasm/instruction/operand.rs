use crate::OPCode;
fn slice_min(bytecode: &[u8], max: usize) -> &[u8] {
    let min = std::cmp::min(bytecode.len(), max);
    &bytecode[..min]
}

pub(crate) fn operand<'a>(opcode: OPCode, bytecode: &'a[u8]) -> Option<&'a [u8]> {
    match opcode {
        OPCode::PUSH1 => Some(slice_min(bytecode, 1)),
        OPCode::PUSH2 => Some(slice_min(bytecode, 2)),
        OPCode::PUSH3 => Some(slice_min(bytecode, 3)),
        OPCode::PUSH4 => Some(slice_min(bytecode, 4)),
        OPCode::PUSH5 => Some(slice_min(bytecode, 5)),
        OPCode::PUSH6 => Some(slice_min(bytecode, 6)),
        OPCode::PUSH7 => Some(slice_min(bytecode, 7)),
        OPCode::PUSH8 => Some(slice_min(bytecode, 8)),
        OPCode::PUSH9 => Some(slice_min(bytecode, 9)),
        OPCode::PUSH10 => Some(slice_min(bytecode, 10)),
        OPCode::PUSH11 => Some(slice_min(bytecode, 11)),
        OPCode::PUSH12 => Some(slice_min(bytecode, 12)),
        OPCode::PUSH13 => Some(slice_min(bytecode, 13)),
        OPCode::PUSH14 => Some(slice_min(bytecode, 14)),
        OPCode::PUSH15 => Some(slice_min(bytecode, 15)),
        OPCode::PUSH16 => Some(slice_min(bytecode, 16)),
        OPCode::PUSH17 => Some(slice_min(bytecode, 17)),
        OPCode::PUSH18 => Some(slice_min(bytecode, 18)),
        OPCode::PUSH19 => Some(slice_min(bytecode, 19)),
        OPCode::PUSH20 => Some(slice_min(bytecode, 20)),
        OPCode::PUSH21 => Some(slice_min(bytecode, 21)),
        OPCode::PUSH22 => Some(slice_min(bytecode, 22)),
        OPCode::PUSH23 => Some(slice_min(bytecode, 23)),
        OPCode::PUSH24 => Some(slice_min(bytecode, 24)),
        OPCode::PUSH25 => Some(slice_min(bytecode, 25)),
        OPCode::PUSH26 => Some(slice_min(bytecode, 26)),
        OPCode::PUSH27 => Some(slice_min(bytecode, 27)),
        OPCode::PUSH28 => Some(slice_min(bytecode, 28)),
        OPCode::PUSH29 => Some(slice_min(bytecode, 29)),
        OPCode::PUSH30 => Some(slice_min(bytecode, 30)),
        OPCode::PUSH31 => Some(slice_min(bytecode, 31)),
        OPCode::PUSH32 => Some(slice_min(bytecode, 32)),
        _ => None,
    }
}
