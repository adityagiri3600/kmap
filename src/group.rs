use std::vec;

pub struct Group {
    pub minterms: Vec<u32>,
    #[allow(dead_code)]
    variables: Vec<u8>,
}

impl Group {
    pub fn new(variables: Vec<u8>) -> Group {
        Self::build_from_variables(variables)
    }

    fn build_from_variables(variables: Vec<u8>) -> Group {
        let mut minterms = vec![0];
        for i in 0..variables.len() {
            match variables[i] {
                1 => {
                    if minterms.len() == 0 {
                        minterms.push(2_u32.pow(variables.len() as u32 - i as u32 - 1));
                    } else {
                        for j in 0..minterms.len() {
                            minterms[j] += 2_u32.pow(variables.len() as u32 - i as u32 - 1);
                        }
                    }
                }
                0 => {
                    if minterms.len() == 0 {
                        minterms.push(0);
                        minterms.push(2_u32.pow(variables.len() as u32 - i as u32 - 1));
                    } else {
                        for j in 0..minterms.len() {
                            minterms.push(minterms[j] + 2_u32.pow(variables.len() as u32 - i as u32 - 1));
                        }
                    }
                }
                _ => {}
            }
        }
        Group {
            minterms,
            variables,
        }
    }
}
