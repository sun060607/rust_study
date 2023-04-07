fn main() {
    let mut number = 10;
    //이렇게 하면 안됨됨
    /*
    let number_ref = &number;
    let number_change = &mut number;
    *number_change +=10;
    println!("{}",number_ref);
    */
    //이렇게 하면 어째서인지 실행이 된다. 에러 없이
    //먼저 포인터를 사용하여 값을 변경하고 난 이후에 사용하였기 때문에 사용 가능
    //이게 되네
    let number_change = &mut number;
    *number_change+=10;
    let number_ref = &number;
    println!("{}",number_ref);
    //----------------------------
    //shadowinh
    //let country = "my country";
    //let country = 8;
    let country = "대한민국";
    let country_ref = &country;
    let country = 8;
    println!("{}, {}",country_ref,country);
}