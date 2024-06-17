fn main() {

    // let 변수명: 타입 = 값;

    let name = "GitHub Copilot"; // 타입 추론을 사용하여 String 타입으로 선언
    let age: u32 = 1; // 명시적 타입 선언


    let mut age = 25; // 가변 변수 선언
    age = 26; // 가변 변수이므로 값 변경 가능


    let (x, y, z) = (1, 2, 3); // x = 1, y = 2, z = 3
    // u32
    println!("Hello World!");
    
    


    // char = 4 bytes

    let first_letter = 'A';
    let space = ' '; // A space inside ' ' is also a char
    let other_language_char = 'Ꮔ'; // Thanks to Unicode, other languages like Cherokee display just fine too
    let cat_face = '😺'; // Emojis are chars too
    
    // This means that Rust can safely cast a u8 into a char, using as. ("Cast u8 as char" means "pretend u8 is a char")

}