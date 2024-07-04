// C의 경우... for (int i = 1; i < 100; i++)  
// 당연하지만 i 100일떄는 처리 못함
// 문법적으로 실수하기가 쉽다

// 러스트의 경우 for문에 오직 이터레이터만 사용
// 최신 C++ 문법과도 유사하다. 

fn fizzbuzz_if_else(max: i32) {
    for i in 1..=max {
        let rem_three: i32 = i % 3;
        let rem_five: i32 = i % 5;

        if rem_three == 0 && rem_five == 0 {
            println!("{} - FizzBuzz", i);
        } else if rem_three == 0 {
            println!("{} - Fizz", i);
        } else if rem_five == 0 {
            println!("{} - Buzz", i);
        } else {
            /* do nothing */
            // 여기 else에 따로 할 일이 없다 해도  "아무런 처리를 하지 않는다"
            // 는 코멘트를 넣어줘야 신경 쓴 코드가 된다..
        }
    }
}


// review

// rust variables

let v_name: i64 = 2147483642;




