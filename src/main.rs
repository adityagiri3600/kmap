mod kmap;
mod group;
use kmap::Kmap;

fn main() {
    let kmap = Kmap::new(4, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    kmap.print_solution();
}
