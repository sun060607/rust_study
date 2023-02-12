fn main(){
    //u32 한계가 없음
    
    //char
    let first_letter = 'A';
    let space = ' ';
    let other_language_char = 'q';
    let cat_face = 'ㅇ';
    //아스키코드에 없는 것을 casting = simple type change
    //타입이 다른 경우 뒤에 바꾸고 싶은 변수에다 as를 넣고 타입을 넣어 더하는 등 연산을 할 수 있다
    let my_number:u16 = 8;
    let second_number: u8 = 10;
    let third_number = my_number+second_number as u16;
    //u8 = 255;
    //간단한 단어만 바꿀 수 있음.
    let my_lan = 'a' as u8;
    println("my lan is{}",my_lan);
}