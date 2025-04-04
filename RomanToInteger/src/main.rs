pub fn roman_to_int(s: String) -> i32 {
    let _roman_to_int = |c : char|-> i32 {
        match c {

            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    };

    let mut total : i32 = 0;
    let chars: Vec<char> = s.chars().collect();
    let n :usize = chars.len();

    for i in 0..n{
        let current_value =_roman_to_int(chars[i]);
        if i< n-1 && current_value < _roman_to_int(chars[i + 1]){
            total -= current_value;
        }
        else {
            total += current_value;
        }
    }

    total
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {

        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {

    // let hell: String = String::from("asdjhfsdjk fsdjkgs dgksldj gsk djg");
    // let n = first_word(&hell);
    // println!("{n}");
    println!("{}", roman_to_int("III".to_string()));
    println!("{}", roman_to_int("LVIII".to_string()));
    println!("{}", roman_to_int("MCMXCIV".to_string()));
}

mod tests{
    use super::*;

    #[test]
    fn test_roman_to_int(){
        let rome_dig_3 = "III".to_string();
        let rome_dig_58 = "LVIII".to_string();
        let rome_dig_1994 = "MCMXCI".to_string();

        assert_eq!(roman_to_int(rome_dig_3.clone()), 3);
        assert_eq!(roman_to_int(rome_dig_58.clone()), 58);
        assert_eq!(roman_to_int(rome_dig_1994.clone()), 1991);
    }
}