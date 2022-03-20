use fixedbitset::FixedBitSet;

fn main() {
    println!("Hello, world!");

    let x = vec![31];
    let bs = FixedBitSet::with_capacity_and_blocks(4, x);

    println!("{:b}", bs);
}
