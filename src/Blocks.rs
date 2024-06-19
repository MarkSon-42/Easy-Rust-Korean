fn main() {
    let my_number = 8; // 
    println!("{}", my_number); 
    {
        let my_number = 9.2; 
        println!("{}", my_number) 
    }
    println!("{}", my_number); 
}