fn main() {
    // + plus sign
    // - minus sign
    // i8, i16, i32, i64, i128, and isize.
    // u8, u16, u32, u64, u128, and usize.


    // unsigned
    // signed Integer
    // bits = 8bit = 1byte

    // computer architecture에 따른 것
    // isize -> 32비트 -> i32
    
    let my_number = 100; // i8, i16 .. .? 알수가 없엉. 우선은 i32 default


    let my_number: u8 = 100;  // 255
    let my_other_number = 200; // i32

    let third_number = my_number + my_other_number;

    // 이렇게 다른 타입 더하기 안됩니다아. 컴파일러가 거부함.
    //type inference


}