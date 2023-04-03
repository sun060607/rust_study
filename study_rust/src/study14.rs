//ownership - 소유권
//& = reference
//그러나 이건 안되는 방법
//주소 참조는 rust에서는 아직 배운 내용에서는 사용할 수 없다.

fn return_it() -> &String{
    let country = String::from("대한민국");
    &country // return &String
}

fn main(){
    //c에서 포인터와 같은 개념이라고 생각하면 편함
    //
    let country = String::from("대한민국");
    let ref_one = &country;
    let ref_two = &ref_one;

    println!("Country is: {}",ref_one);

    let my_country = return_it();
}