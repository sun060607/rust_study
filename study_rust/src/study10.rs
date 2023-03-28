fn main(){
    //특수 문자 출력에 대한 설명임 c와 거의 유사\
    //r#""#을 사용하면 기존의 c에서 쓰는 두번 쓰고 쓰지 않아도 그대로 출력
    print!(r#"C:\Users\sung0\Desktop\rust\study_rust\src"#);
    //이런 형식으로 하면 \n이 된다 (어째서?)
    print!("
        어떤 이야기를 
        봅시다
    ");
    //포인터 주소를 출력하고 싶으면 {:p}를 이용하여 출력 가능
    //{:x}는 다시 찾고 {:b}는 2진수로 표현하여 출력
    let my_number = &9;
    println!("{:p}",my_number);
    //^은 중앙에 싶은 문자나 문자열(수)를 넣어 정렬 뒤의 숫자는 문자 문자열 포함 글자수
    //^,<,> 앞에 스페이스나 -를 넣으면 그걸 나머지 출력으로 함
    //<
    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // no variable name, pad with -, put in centre, 30 characters long
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 characters each, one to the left, one to the right
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b); // variable names city1 and city2, pad with -, one to the left, one to the right

}