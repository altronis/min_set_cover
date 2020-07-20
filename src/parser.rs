use std::fs;
use std::str::Lines;
use bit_set::BitSet;

pub fn form_set(nums: &str, nbits: usize) -> Option<BitSet> {
    let mut set: BitSet = BitSet::with_capacity(nbits);

    for element in nums.trim().split(" ") {
        let element_num: usize = match element.parse() {
            Ok(num) => num,
            Err(_) => {
                return None;
            }
        };

        set.insert(element_num);
    }

    Some(set)
}

pub fn get_sets(filename: &str) -> Result<Vec<BitSet>, &'static str> {
    // Read the contents of the file into a single string
    let contents: String = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(_) => {
            return Err("Invalid file name");
        }
    };

    let mut lines_iter: Lines = contents.lines();

    // The first line of the file contains the total no. of elements
    let n: &str = match lines_iter.next() {
        Some(s) => s,
        None => {
            return Err("Couldn't read number of elements");
        }
    };

    let n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return Err("Couldn't parse file");
        }
    };

    let mut sets: Vec<BitSet> = Vec::new();

    // Read the sets
    for line in lines_iter {
        let set: Option<BitSet> = form_set(line, n);

        match set {
            Some(s) => {
                sets.push(s);
            }
            None => {
                return Err("Couldn't parse file");
            }
        }
    }

    Ok(sets)
}