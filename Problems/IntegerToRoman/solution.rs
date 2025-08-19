use std::collections::HashMap;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let roman_map: HashMap<i32, &str> = HashMap::from([
            (1, "I"),
            (2, "II"),
            (3, "III"),
            (4, "IV"),
            (5, "V"),
            (6, "VI"),
            (7, "VII"),
            (8, "VIII"),
            (9, "IX"),
            (10, "X"),
            (20, "XX"),
            (30, "XXX"),
            (40, "XL"),
            (50, "L"),
            (60, "LX"),
            (70, "LXX"),
            (80, "LXXX"),
            (90, "XC"),
            (100, "C"),
            (200, "CC"),
            (300, "CCC"),
            (400, "CD"),
            (500, "D"),
            (600, "DC"),
            (700, "DCC"),
            (800, "DCCC"),
            (900, "CM"),
            (1000, "M"),
            (2000, "MM"),
            (3000, "MMM"),
        ]);
        let num_len = num.checked_ilog10().unwrap_or(0) + 1;
        let mut answer: String = "".to_string();
        for i in (0..num_len).rev() {
            let digit = ((num / 10_i32.pow(i)) % 10) * 10_i32.pow(i);
            if digit == 0 {
                continue;
            }
            answer += roman_map.get(&digit).unwrap();
        }
        answer.to_string()
    }
}
//this was my initial solution but one of the solutions I found with brute force was funny so I decided to add it, it is also faster and uses less space than mine.
fn second_way(num: i32) -> String {
    const ONES: [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    const TENS: [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    const HUNDREADS: [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    const THOUSANDS: [&str; 4] = ["", "M", "MM", "MMM"];
    format!(
        "{}{}{}{}",
        THOUSANDS[(num / 1000 % 10) as usize],
        HUNDREADS[(num / 100 % 10) as usize],
        TENS[(num / 10 % 10) as usize],
        ONES[(num % 10) as usize],
    )
}

//another way of doing it I learned since this is for learning the greedy approach

fn third_way(num: i32) -> String {
    let values = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut result = String::new();

    for (value, symbol) in &values {
        while num >= *value {
            result.push_str(symbol);
            num -= value;
        }
    }

    result
}
