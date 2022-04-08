#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub fn if_statement() {
    let temp = 25;

    if temp>30 {
        println!("Temperature is hot");
    } else if temp < 10 {
        println!("Temperature is cold!");
    } else {
        println!("Temperature is nice!")
    }

    let day = if temp > 20 {"Sunny"} else {"Cloudy"};
    println!("Day is {}", day);
}


#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub fn loops() {
    let mut x = 1;
    while x < 1000 {
        x *=2;
        if x == 64 {
            continue;
        }
        println!("x = {x}");
    }
    let mut y = 1;
    loop { // while true
        y *= 2 ;
        println!("y = {}", y);
        if y == 1 << 10 {break}
    }

    for x in 1..11 { // [1, 10], [1, 11)
        if x == 3 {continue}
        if x == 8 {break}
        println!("x = {x}")
    }

    for (pos, y) in (30..41).enumerate() {
        println!("Index = {pos}, y = {y}")
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused)]
pub fn match_statement() {
    let country_code = 1001;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _ => "invalid" // if we don't cover others, we will get non-exhaustive pattern error!.
    };

    println!("Country Code = {}, Country = {}", country_code, country);
}
