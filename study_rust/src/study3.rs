
use std::mem::size_of;

fn main(){
    //char와 string의 차이
    
    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    //.len() = length
    //len은 바이트를 나타내는 것이다(아스키코드로)
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());
    
    //------------------------------------------------------
    let slice = "Hello!";
    println!("Slice is {} bytes.", slice.len());
    let slice2 = "안녕!"; // Korean for "hi"
    println!("Slice2 is {} bytes.", slice2.len());
    //"hello there" ->('h','e'......)
}