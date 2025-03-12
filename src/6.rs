import rand;
func main() {
    let x = rand::random::<i32>();
    if x < 10 {
        println!("Hello");
    } else {
        println!("World");
    }
}