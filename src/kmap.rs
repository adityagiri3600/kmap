use std::process::exit;

use crate::group::Group;
use itertools::Itertools;

pub struct Kmap {
    n: u32,
    minterms: Vec<u32>,
}

impl Kmap {
    pub fn new(n: u32, minterms: Vec<u32>) -> Kmap {
        minterms.iter().for_each(|minterm| {
            if *minterm >= 2_u32.pow(n) {
                println!(
                    "Minterm {} is out of bounds for a Kmap with {} variables",
                    minterm, n
                );
                exit(1)
            }
        });
        Kmap {
            n,
            minterms: minterms.clone(),
        }
    }

    pub fn solve(&self) -> Vec<Group> {
        let mut groups = vec![];
        let mut used_minterms = vec![];
        for size in 0..=self.n {
            loop {
                let mut max_score = 0;
                let mut best_group = Group::new(vec![2, 2, 2, 2]);
                // println!("Size: {}", size);
                for variables_to_use in (0..self.n).combinations(size as usize) {
                    for combination in vec![1..=2; size as usize]
                        .iter()
                        .cloned()
                        .multi_cartesian_product()
                    {
                        let mut variables = vec![0 as u8; self.n as usize];

                        for i in 0..size {
                            variables[variables_to_use[i as usize] as usize] =
                                combination[i as usize] as u8;
                        }
                        let group = Group::new(variables);
                        // println!("{:?}", variables);
                        // println!("{:?}", group.minterms);
                        // println!("{:?}", self.minterms);
                        if group.minterms.iter().all(|minterm| {
                            self.minterms.contains(minterm) || used_minterms.contains(minterm)
                        }) {
                            // print!("found: {:?}", group.minterms);
                            let mut score = 0;
                            for minterm in &group.minterms {
                                if self.minterms.contains(minterm)
                                    && !used_minterms.contains(minterm)
                                {
                                    score += 1;
                                }
                            }
                            if score > max_score {
                                max_score = score;
                                best_group = group;
                            }
                        }
                    }
                }
                if max_score == 0 {
                    break;
                }
                groups.push(best_group.clone());
                used_minterms.extend(best_group.minterms.iter());
            }
        }
        groups
    }

    #[allow(dead_code)]
    pub fn expression(&self) -> String {
        let groups = self.solve();
        let mut solution = String::new();
        solution.push_str(&format!("F = {}", groups[0].expression()));
        for group in groups.iter().skip(1) {
            solution.push_str(&format!(" + {}", group.expression()));
        }
        solution
    }
}
