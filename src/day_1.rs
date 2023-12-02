pub fn part_1(input: String) {
    let mut result = 0;
    for line in input.lines() {
        let mut first = None;
        let mut last = None;

        for char in line.chars() {
            if char.is_ascii_digit() {
                if first.is_none() {
                    first = Some(char);
                }
                last = Some(char);
            }
        }

        let two_digits = match (first, last) {
            (None, None) => "0".to_string(),
            (None, Some(last)) => last.to_string(),
            (Some(first), None) => first.to_string(),
            (Some(first), Some(last)) => format!("{first}{last}"),
        };

        if let Ok(number) = two_digits.parse::<u32>() {
            result += number;
        }
    }

    println!("{result}");
}

// There are probably easier ways (with or without a recursive descent parser)
peg::parser! {
    grammar parser() for str {
        rule one() -> Option<u32> = "one" { Some(1) }
        rule two() -> Option<u32> = "two" { Some(2) }
        rule three() -> Option<u32> = "three" { Some(3) }
        rule four() -> Option<u32> = "four" { Some(4) }
        rule five() -> Option<u32> = "five" { Some(5) }
        rule six() -> Option<u32> = "six" { Some(6) }
        rule seven() -> Option<u32> = "seven" { Some(7) }
        rule eight() -> Option<u32> = "eight" { Some(8) }
        rule nine() -> Option<u32> = "nine" { Some(9) }
        rule digit() -> Option<u32> = n:$(['0'..='9']) { n.parse().ok() }

        rule other() -> Option<u32> = ['a'..='z' | 'A'..='Z'] { None }

        rule oneight() -> Option<u32> = "oneight" { Some(1) }
        rule oneight_last() -> Option<u32> = "oneight" other()* ![_] { Some(8) }     
        
        rule twone() -> Option<u32> = "twone" { Some(2) }
        rule twone_last() -> Option<u32> = "twone" other()* ![_] { Some(1) } 

        rule threeight() -> Option<u32> = "threeight" { Some(3) }
        rule threeight_last() -> Option<u32> = "threeight" other()* ![_] { Some(8) }

        rule fiveight() -> Option<u32> = "fiveight" { Some(5) }
        rule fiveight_last() -> Option<u32> = "fiveight" other()* ![_]  { Some(8) }        

        rule sevenine() -> Option<u32> = "sevenine" { Some(7) }
        rule sevenine_last() -> Option<u32> = "sevenine" other()* ![_]  { Some(9) }
        
        rule eightwo() -> Option<u32> = "eightwo" { Some(8) }
        rule eightwo_last() -> Option<u32> = "eightwo" other()* ![_]  { Some(2) }
                
        rule eighthree() -> Option<u32> = "eighthree" { Some(8) }
        rule eighthree_last() -> Option<u32> = "eighthree" other()* ![_]  { Some(3) }

        rule nineight() -> Option<u32> = "nineight" { Some(9) }
        rule nineight_last() -> Option<u32> = "nineight" other()* ![_]  { Some(8) }

        rule matching() -> Option<u32> = oneight_last() / oneight() / one() / twone_last() / twone() / two() / threeight_last() / threeight() / three() / four() / fiveight_last() / fiveight() / five() / six() / sevenine_last() / sevenine() / seven() / eightwo_last() / eightwo() / eighthree_last() / eighthree() / eight() / nineight_last() / nineight() / nine() / digit()

        pub rule calibration() -> Vec<Option<u32>> = (matching() / other())*
    }
}

pub fn part_2(input: String) {
    let mut result = 0;
    for line in input.lines() {
        if let Ok(calibration) = parser::calibration(line)
            .map(|calibration| calibration.into_iter().flatten().collect::<Vec<_>>())
        {
            if let (Some(first), Some(last)) = (calibration.first(), calibration.last()) {
                result += first * 10 + last;
            }
        }
    }

    println!("{result}");
}
