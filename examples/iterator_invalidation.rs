
fn main() {
    let mut xs: Vec<u32> = vec![1, 2, 3];
    let x = &xs[0];
    xs.push(4);
    println!("{}", x);
}
