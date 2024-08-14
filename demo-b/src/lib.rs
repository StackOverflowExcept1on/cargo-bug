use demo_a::add;

pub fn sum(a: u64, b: u64, c: u64) -> u64 {
    add(add(a, b), c)
}
