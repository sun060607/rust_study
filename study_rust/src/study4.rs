fn main(){
    //타입에 관한 공부
    let my_number = 9u8;//숫자는 붙여서 쓸 수 있음
    let other_number = 1_000_000_000u64;//이렇게 써도 상관 없음
    //float는 f32, f64를 사용(대부분 f64를 사용)
    let my_number = 9.;//f64
    let other_number = 9;//i32
    println("{}",my_number+other_number as f64);

}