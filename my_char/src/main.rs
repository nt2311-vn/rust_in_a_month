fn main() {
    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("Size of a: {}", "a".len());
    println!("Size of ß: {}", "ß".len());
    println!("Size of 国: {}", "国".len());
    println!("Size of λ : {}", "λ ".len());

    let str1 = "Hello!";
    println!("str1 is {} bytes", str1.len());

    let str2 = "안녕!";
    println!("str2 is {} bytes", str2.len());

    println!("{:?}", "a".as_bytes());
    println!("{:?}", "ß".as_bytes());
    println!("{:?}", "国".as_bytes());
    println!("{:?}", "λ".as_bytes());
}
