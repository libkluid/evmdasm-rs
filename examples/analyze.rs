extern crate evmdasm;
extern crate hex;
extern crate env_logger;

// Contract: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
const MULTICALL2: &'static str = include_str!("bytecodes/eth-uniswap-v2-router-2.txt");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let bytes = hex::decode(MULTICALL2.trim())?;

    let dasm = evmdasm::Disassembler::new(&bytes);
    let analyzer = evmdasm::CodeAnalyzer::new(dasm);

    analyzer.analyze();

    Ok(())
}
