fn main() {
    let days_of_xmas: [&str; 12] = [
        "a partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "gold rings!",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers pipping",
        "drummers drumming"
    ];

    for day in 1..13 {
        let day_postfix: &str = 
            if day == 1 {
                "st"
            } else {
                "th"
            };
        let day_name = if day > 1 { 
            let mut day_name = String::from(day.to_string());
            day_name.push_str(" ");
            day_name
        } else { String::from("") };
        println!(
            "On the {}{} day of Christmas my true love gave to me, {}{}",
            day,
            day_postfix,
            day_name,
            days_of_xmas[day - 1]
        );
        for previous_day in (1..day).rev() {
            let day_name = if previous_day > 1 { 
                let mut day_name = String::from(previous_day.to_string());
                day_name.push_str(" ");
                day_name
            } else { String::from("and ") };

            println!(
                "{}{}",
                day_name,
                days_of_xmas[previous_day - 1]
            )
        }
    }
}
