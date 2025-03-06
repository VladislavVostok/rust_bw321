
fn my_atoi(s: String) -> i32{
    let mut chars = s.chars().peekable();
    let mut result: i64 = 0;
    let mut sign = 1;

    while let Some(&' ') = chars.peek(){
        chars.next();
    }

    match chars.peek(){
        Some(&'+') => {
            chars.next();
        }
        Some(&'-') => {
            sign = -1;
            chars.next();
        }
        _ => {}
    }

    while let Some(&c) = chars.peek(){
        if c.is_ascii_digit(){
            result = result * 10 + (c as i64 - '0' as i64);
            chars.next();

            if result * sign > i32::MAX as i64{
                return i32::MAX;
            }

            if result * sign < i32::MIN as i64{
                return i32::MIN;
            }
        }
        else {
            break;
        }
    }



    (result * sign) as i32
}

fn main() {

    println!("{}", my_atoi("42".to_string()));
    println!("{}", my_atoi(" -042".to_string()));
    println!("{}", my_atoi("1337c0d3".to_string()));
    println!("{}", my_atoi("0-1".to_string()));
    println!("{}", my_atoi("words and 987".to_string()));
    println!("{}", my_atoi("-91283472332".to_string()));
}
