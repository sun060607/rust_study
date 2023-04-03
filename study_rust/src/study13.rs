//const는 무조건 개문자로 해야지 경고가 나오지 않는다
//약간 전역 변수 같은 느낌
const HIGH_SCORE: i32 = 20;
//attribute
//이걸 사용하면 소문자로도 가능하다
//어째서?
//#[allow(non_upper_case_globals)]
fn print_high_school(){
    println!("가장 큰 점수는 {}",HIGH_SCORE);
}
//같은 메모리에서 사용됨
//꼭 쓸려면 mut를 붙여줘야함
static mut LOW_SRCORE: i32 = 0;//unsafe
//static lifetime
//지역 변수와 전역 변수의 차이를 이해하면 const를 이해할 수 있음
fn main(){
    let x = 8;//let binding: i32
    print_high_school();
    //static을 사용하려면 unsafe를 사용해야 함
    //최대한 사용하지 말아야 하는 것
    //unsafe
    unsafe{
        LOW_SRCORE = 1;
    }
}