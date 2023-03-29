fn main(){
    //owned type
    //사이즈 크기를 알 수 있다
    //string = Sized type
    //str = dynamic type
    
    //String 커질 수 있고 낮아질수 있고
    //$str ref str "sting slice"
    //name: String,
    //address: String
    let my_name = "minsung".to_string();//&str //string을 변환
    let other_name = String::from("minsung");
    //growable + shrinkable 
    //추가 가능
    let mut my_other_name = "minsung".to_string();
    my_other_name.push('!');
    println!("{}",my_other_name);
}