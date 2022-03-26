extern crate evmdasm;
extern crate hex;

use std::fmt::Write;

// Contract: 0x5BA1e12693Dc8F9c48aAD8770482f4739bEeD696
const MULTICALL2: &'static str = include_str!("bytecodes/eth-multicall2.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = hex::decode(MULTICALL2.trim())?;

    let dasm = evmdasm::Disassembler::new(&bytes);
    let instructions = dasm.disassemble();

    for ins in instructions {
        let mut buffer = String::new();
        write!(&mut buffer, "{:>08X}| {:X?}", ins.offset, ins.opcode)?;
        if let Some(operand) = ins.operand {
            write!(&mut  buffer, " 0x{}", hex::encode(operand))?;
        }
        println!("{}", buffer);
    }

    Ok(())
}
