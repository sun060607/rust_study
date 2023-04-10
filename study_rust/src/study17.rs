//ownership
//move semantics
fn print_country(country_name: &String)/*->String*/{
    println!("my country is {}",country_name);
    //country_name
}

fn main(){
    let country = "대한민국".to_string();
    //country = print_country(country);
    //country = print_country(country);
    //country = print_country(country);
    print_country(&country);
    print_country(&country);
    print_country(&country);
    //만약에 펑션을 다시 사용할 경우 에러 발생
    //이유 소유권으로 인해 소유했다가 다 끝나면 삭제 되어서 에러

    //print_country(country); 
}