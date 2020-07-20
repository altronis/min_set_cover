mod parser;

use bit_set::BitSet;

pub fn run(filename: &str) -> Result<(), &'static str> {
    let sets: Vec<BitSet> = parser::get_sets(filename)?;

    Ok(())
}