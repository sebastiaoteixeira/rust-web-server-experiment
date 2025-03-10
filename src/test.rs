fn main() {
    let mut a: i32 = 5;

    {
        let b = &mut a;

        increment(b);
        *b = 10;
        println!("{}", b);
    };

    let mut vec = Vec::new();

    vec.push(1u32);
    vec.push(2u32);
    vec.push(3u32);

    let str: String = "Hello, world!".to_string();
    let len = calculate_length(&str);
    println!("{}", len);

    let str2: String = str.clone();
    println!("{}", str2);
    println!("{}", str);

    println!("{}", stringify!(1 + 2));

    println!("{}", a);
}

fn increment(a: &mut i32) {
    *a += 1;
}

fn calculate_length(s: &str) -> usize {
    s.len()
}
