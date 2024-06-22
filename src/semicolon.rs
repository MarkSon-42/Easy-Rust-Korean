// display 트레잇과 debug 트레잇

fn empty_tuple() {

}


// Display {}

// Debug print
fn main() {
    let tuple = empty_tuple();
    println!("{:?}", tuple);
    // {:?} 는 포맷지정자를 의미한다
}