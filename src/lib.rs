use wasm_bindgen::prelude::*;
mod kmap;
mod group;
use kmap::Kmap;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn solve_kmap(number_of_variables: u32, minterms: Vec<u32>) -> String {
    let kmap = Kmap::new(number_of_variables, minterms);
    let groups = kmap.solve();
    let mut result = String::new();
    result.push_str(groups.len().to_string().as_str());
    result.push_str("\n");
    for group in groups {
        for i in 0..group.minterms.len() {
            result.push_str(&format!("{} ", group.minterms[i]));
        }    
        result.push_str("\n");
        result.push_str(&format!("{}\n", group.expression()));
    }
    result
}