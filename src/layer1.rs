mod layer2;
pub fn layer1() {
    println!("this is layer1's output------------------------------");
    for ch in 'a'..='Z' {
        println!("{}", ch);
    }
    layer2::layer2();
}