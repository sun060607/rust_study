fn main(){ 
//reallocation
//공간을 추가하는 것으로 일정 수치 이상으로 가면 바이트를 2배 올린다
//이건 약간 이론 느낌
//------------------------------------------
//String
    //.capacity 
    //.push
    //.push str
    //.pop
    //.with_capacity
    //allocation 
    //let my_name = "".to_string();
    //이렇게 할 경우 난 처음부터 늘리지 않고 26byte만 쓰겠다는 것을 선언하는 것이다
    //허나 이것도 초과하게 되면 byte 크기는 2배로 커진다
    
    let mut my_name = String::with_capacity(26);
    //이걸 사용하면 문자열 뒤에 push 해서 넣어줌
    //파이썬과 비슷한 기능이라고 생각하면 됨
    //capacity는 byte를 얼마나 할당할 것인가 이며 
    //len은 현재 그 값이 얼마나 byte를 사용하는 지 알려준다
    println!("{} {}",my_name.len(),my_name.capacity());
    my_name.push_str("minsung!");//그냥 push는 문자 추가
    println!("{} {}",my_name.len(),my_name.capacity());
    //이걸 사용하면 문자열에 문자열을 추가 가능
    my_name.push_str("and I live in busen");//문자열 추가
    println!("{} {}",my_name.len(),my_name.capacity());
}