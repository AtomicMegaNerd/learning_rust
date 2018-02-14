pub fn run_more_one() {
    let tshirt_width = 20;
    let tshirt_size = match tshirt_width {
        16 => "S",
        17 | 18 => "M",
        19...21 => "L",
        22 => "XL",
        _ => "Not available",
    };
    println!("{}", tshirt_size);
}
