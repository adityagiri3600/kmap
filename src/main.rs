mod group;
use group::Group;

fn main() {
    let group = Group::new(vec![2,2,2,1]);
    println!("{:?}", group.minterms);
}
