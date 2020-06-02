use bigdecimal::BigDecimal;
use std::str::FromStr;

fn main() {
    let mut sum = BigDecimal::from_str("0").unwrap();
    let unit = BigDecimal::from_str("0.1").unwrap();

    let mut sum_float: f64 = 0.0;
    let unit_f: f64 = 0.1;

    for _ in 0..100 {
        sum_float += unit_f;
        sum += &unit;
    }

    println!("f64        {}", sum_float);
    println!("bigdecimal {}", sum);
}
