mod more1;

fn add_nums(a: i64, b: i64) -> i64 {
    // Add two numbers together and return the result
    return a + b;
}

fn func_ptrs() {
    const ROFL: i64 = 99;
    let t = ROFL + 1;
    let xray = add_nums(32, 41);
    let alias1 = add_nums;
    let alias2: fn(i64, i64) -> i64 = add_nums;
    let t2 = alias1(3, 4);

    println!("{0} {1} {2} {3}", t, xray, t2, alias2(3, 4));
}

fn main() {
    // This is a comment
    let x = 3;
    let y = 4;

    let z = x + y;

    let s1 = "Hello there";
    let s2 = "I am coding in Rust";

    println!("Hello, world!");
    println!("z = {}", z);

    println!("{0} + {1} = {2}", x, y, z);

    println!("{0} {1}", s1, s2);
    println!("{:?}", [s1, s2]);

    func_ptrs();

    println!("{0}", u8::min_value());

    let arr1 = [1, 2, 3, 4];
    let arr2: [char; 5] = ['H', 'e', 'l', 'l', 'o'];

    println!("{:?}", arr1);
    println!("{:?}", arr2);

    let tup1 = (1, 4.2, "rofl", 'c');
    println!("{:?}", tup1);

    // Slicing
    let a: [i32; 4] = [1, 2, 3, 4];

    // Slice the whole array
    let b: &[i32] = &a;
    println!("{:?}", b);

    let c = &a[0..4]; // From 0 up to (excluding) 4 ix
    println!("{:?}", c);

    let chstr = "你好，我性丁";
    let ch_heap_str = String::from(chstr);

    println!("{0}", ch_heap_str);

    let b: &str = "This is another string";
    println!("{0}", b);

    // type casting!
    let sauce = 15;
    let roar = (sauce as f64) / 2.0;

    println!("{0}", roar);

    let team_size = 7;
    if team_size < 5 {
        println!("Small");
    } else if team_size < 10 {
        println!("Medium");
    } else {
        println!("Large");
    }

    let time_size_in_text = if team_size < 5 {
        "Small"
    } else if team_size < 10 {
        "Medium"
    } else {
        "Large"
    };
    println!("Current team size : {}", time_size_in_text);

    more1::run_more_one();
}
