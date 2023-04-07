//& immutable reference / shared reference
//&mut mutable reference / unique reference
//& *

fn main(){
    let mut my_number = 9;

    let num_ref = &mut my_number;
    //타입이 달라서 그냥은 사용이 안됨
    //포인터처럼 사용해야 변경됨(어째서?)
    *num_ref = 10;

    println!("{}",my_number)
}