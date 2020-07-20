mod parser;

use bit_set::BitSet;

fn remove_subsets(sets: Vec<BitSet>) -> Vec<BitSet> {
    let num_sets: usize = sets.len();
    let mut new_sets: Vec<BitSet> = Vec::new();

    // Keep sets that are not subsets of other sets
    for i in 0..num_sets {
        let set1: &BitSet = &sets[i];
        let mut is_subset: bool = false;

        for j in 0..num_sets {
            let set2: &BitSet = &sets[j];

            if set1 != set2 && set1.is_subset(set2) {
                is_subset = true;
                break;
            }
        }

        if !is_subset {
            new_sets.push((*set1).clone());
        }
    }

    new_sets
}

fn find_essential_sets(sets: &Vec<BitSet>) -> Vec<bool> {
    let mut essential: Vec<bool> = Vec::new();
    let num_sets: usize = sets.len();

    // Find which sets must appear in the solution
    for i in 0..num_sets {
        let mut union = BitSet::new();

        // Find the union of every other set
        for j in 0..num_sets {
            if i != j {
                union.union_with(&sets[j]);
            }
        }

        essential.push(!sets[i].is_subset(&union));
    }

    essential
}

pub fn run(filename: &str) -> Result<(), &'static str> {
    let (mut sets, n): (Vec<BitSet>, usize) = parser::get_sets(filename)?;
    sets = remove_subsets(sets);

    // Sort the sets so the bigger sets come first
    sets.sort_unstable_by(|a, b| a.len().cmp(&b.len()).reverse());
    let essential: Vec<bool> = find_essential_sets(&sets);

    let mut a: Vec<bool> = Vec::new();
    a.resize(sets.len() + 1, false);

    Ok(())
}