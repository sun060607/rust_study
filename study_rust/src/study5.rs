//macro = function that writes code
//복잡한 코드일 경우에 많이 씀
//함수 뒤에 타입을 쓰면 return 역할을 함
fn give_age() -> i32{
    42
    
}


fn main(){
    println!("hello world");
    let my_number = "David";
    //let my_age = 42;
    //무조건 넣어야 함(그냥 c랑 비슷함)
    println("{}{}",my_number,give_age);
    //println("{my_number}{}",my_number,give_age);
    //버젼 1.58 부터 쓸 수 있음

}