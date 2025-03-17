use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1, 7);
    println!("The random number is {}", num);
}
