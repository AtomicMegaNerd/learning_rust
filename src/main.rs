mod control_flows;
mod basics;

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

    basics::func_ptrs();

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

    control_flows::matches_one();

    control_flows::matches_two(true);

    control_flows::grade_papers(46, 18);

    control_flows::while_examples();

    control_flows::loop_examples();

    control_flows::for_examples();
}
