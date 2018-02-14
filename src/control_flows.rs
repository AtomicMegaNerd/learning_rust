pub fn matches_one() {
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

pub fn matches_two(is_allowed: bool) {
    let list_type = match is_allowed {
        true => "Full",
        false => "Restricted", // no default/ _ condition can be skipped
                               // Because data type of is_allowed is boolean and all possibilities checked on conditions
    };
    println!("{}", list_type); // Restricted
}

pub fn grade_papers(marks_paper_a: u8, marks_paper_b: u8) {
    let output = match (marks_paper_a, marks_paper_b) {
        (50, 50) => "Full marks for both papers",
        (50, _) => "Full marks for paper A",
        (_, 50) => "Full marks for paper B",
        (x, y) if x > 25 && y > 25 => "Good",
        (_, _) => "Work hard",
    };
    println!("{}", output); // Work hard
}

pub fn while_examples() {
    println!();
    println!("*** while loop examples ***");
    println!();

    let mut a = 1;
    while a <= 10 {
        println!("Current value : {}", a);
        a += 1; //no ++ or -- on Rust
    }

    // Usage of break and continue
    let mut b = 0;
    while b < 5 {
        if b == 0 {
            println!("Skip value : {}", b);
            b += 1;
            continue;
        } else if b == 2 {
            println!("Break At : {}", b);
            break;
        }
        println!("Current value : {}", b);
        b += 1;
    }

    // Outer break
    let mut c1 = 1;
    'outer_while: while c1 < 6 {
        //set label outer_while
        let mut c2 = 1;
        'inner_while: while c2 < 6 {
            println!("Current Value : [{}][{}]", c1, c2);
            if c1 == 2 && c2 == 2 {
                break 'outer_while;
            } //kill outer_while
            c2 += 1;
        }
        c1 += 1;
    }
}

pub fn loop_examples() {
    println!();
    println!("*** loop examples ***");
    println!();

    // loop forever
    // loop {
    //     println!("Loop forever!");
    // }

    // Usage of break and continue
    let mut a = 0;
    loop {
        if a == 0 {
            println!("Skip Value : {}", a);
            a += 1;
            continue;
        } else if a == 2 {
            println!("Break At : {}", a);
            break;
        }
        println!("Current Value : {}", a);
        a += 1;
    }

    // Outer break
    let mut b1 = 1;
    'outer_loop: loop {
        //set label outer_loop
        let mut b2 = 1;
        'inner_loop: loop {
            println!("Current Value : [{}][{}]", b1, b2);
            if b1 == 2 && b2 == 2 {
                break 'outer_loop; // kill outer_loop
            } else if b2 == 5 {
                break;
            }
            b2 += 1;
        }
        b1 += 1;
    }
}

pub fn for_examples() {
    println!();
    println!("*** for loop examples ***");
    println!();

    for a in 0..10 {
        //(a = o; a <10; a++) // 0 to 10(exclusive)
        println!("Current value : {}", a);
    }

    // Usage of break and continue
    for b in 0..6 {
        if b == 0 {
            println!("Skip Value : {}", b);
            continue;
        } else if b == 2 {
            println!("Break At : {}", b);
            break;
        }
        println!("Current value : {}", b);
    }

    // Outer break
    'outer_for: for c1 in 1..6 {
        //set label outer_for
        'inner_for: for c2 in 1..6 {
            println!("Current Value : [{}][{}]", c1, c2);
            if c1 == 2 && c2 == 2 {
                break 'outer_for;
            } //kill outer_for
        }
    }

    // Working with arrays/vectors
    let group: [&str; 4] = ["Mark", "Larry", "Bill", "Steve"];

    for n in 0..group.len() {
        //group.len() = 4 -> 0..4 👎 check group.len()on each iteration
        println!("Current Person : {}", group[n]);
    }

    for person in group.iter() {
        //👍 group.iter() turn the array into a simple iterator
        println!("Current Person : {}", person);
    }
}
