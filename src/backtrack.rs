use bit_set::BitSet;

pub struct Backtracker {
    sets: Vec<BitSet>,

    // essential[i] is true if the ith set MUST appear in the solution
    essential: Vec<bool>,

    // The number of sets found in the solution so far
    pub max: usize,

    // The universal set with n bits.
    total: BitSet,

    // A string representation of the solution
    pub repr: String,

    // The total number of elements
    nbits: usize,
}

fn to_string(set: &BitSet) -> String {
    if set.is_empty() {
        return "{}".to_owned();
    }

    let mut s: String = String::from("{");

    for elem in set.iter() {
        let elem_str: &str = &((elem + 1).to_string());
        s.push_str(elem_str);
        s.push_str(", ");
    }

    format!("{}}}", &s[..s.len() - 2])
}

impl Backtracker {
    pub fn new(sets: Vec<BitSet>, essential: Vec<bool>, nbits: usize) -> Backtracker {
        // total is the universal set
        let mut total: BitSet = BitSet::with_capacity(nbits);

        for i in 0..nbits {
            total.insert(i);
        }

        let num_sets: usize = sets.len();

        Backtracker {
            sets,
            essential,
            max: num_sets,
            total,
            repr: String::new(),
            nbits,
        }
    }

    fn find_union(&self, a: &Vec<bool>, k: usize) -> (BitSet, usize) {
        // Find the union of the first k sets where a[i] is true
        let mut union: BitSet = BitSet::with_capacity(self.nbits);
        let mut count: usize = 0;

        for i in 1..k + 1 {
            if a[i] == true {
                count += 1;
                union.union_with(&self.sets[i - 1]);
            }
        }

        (union, count)
    }

    fn process_solution(&mut self, a: &Vec<bool>, k: usize) {
        let (union, count): (BitSet, usize) = self.find_union(&a, k);

        // Make sure we are covering all elements and better than previous solution
        if self.total == union && count < self.max {
            let mut s: String = String::from("{");

            for i in 1..k + 1 {
                if a[i] == true {
                    s.push_str(&to_string(&self.sets[i - 1]));
                    s.push(' ');
                }
            }

            s.truncate(s.len() - 1);
            s.push(' ');

            self.repr = s;
            self.max = count;
        }
    }

    fn must_appear_prefix(&self, a: &Vec<bool>, k: usize) -> bool {
        // Determine if the kth element is essential to the solution given the sol'n so far
        let mut union: BitSet = BitSet::with_capacity(self.nbits);

        for i in 1..k {
            if a[i] == true {
                union.union_with(&self.sets[i - 1]);
            }
        }

        for i in k + 1..a.len() {
            union.union_with(&self.sets[i - 1]);
        }

        !self.sets[k - 1].is_subset(&union)
    }

    fn construct_candidates(&self, a: &Vec<bool>, k: usize) -> Vec<bool> {
        let (union, count): (BitSet, usize) = self.find_union(&a, k - 1);

        if count > self.max - 1 {
            // We can't do better than the prev solution
            return vec!();
        } else if count >= self.max - 1 || self.total.is_subset(&union) {
            return vec!(false);
        } else if self.essential[k - 1] || self.must_appear_prefix(&a, k) {
            // This set must appear in the solution
            return vec!(true);
        } else if self.sets[k - 1].is_subset(&union) {
            // Including this set won't make the solution better, so exclude it
            return vec!(false);
        }

        vec!(true, false)
    }

    pub fn backtrack(&mut self, a: &mut Vec<bool>, k: usize) {
        if k == self.sets.len() {
            self.process_solution(&a, k);
        } else {
            let new_k: usize = k + 1;
            let c: Vec<bool> = self.construct_candidates(&a, new_k);

            for i in c {
                a[new_k] = i;
                self.backtrack(a, new_k);
            }
        }
    }
}