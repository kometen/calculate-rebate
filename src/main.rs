use bigdecimal::{BigDecimal, FromPrimitive};
use std::str::FromStr;

struct Rebate {
    volume: i32,
    rebate: BigDecimal,
}

impl Rebate {
    fn new(volume: i32, rebate: BigDecimal) -> Self {
        Self {
            volume,
            rebate,
        }
    }
}

fn calculate_rebate_fn(total: i32, rebates: Box<[Rebate]>) -> BigDecimal {
    let mut total_rebate = BigDecimal::from_i32(0).unwrap();
    let _t = BigDecimal::from_i32(total).unwrap();

    let mut rebate = rebates.iter();

    loop {
        match rebate.next() {
            Some(r) => {
                if r.volume > total {
                    break
                } else {
                    let remaining = total - &r.volume + 1;
                    let _r = BigDecimal::from_i32(remaining).unwrap();
                    total_rebate += &_r / &_t * &r.rebate;
                }
            }
            None => break,
        }
    }
    total_rebate
}

fn total_rebate_fn(total: i32) -> BigDecimal {
    let rebates = [
        Rebate::new(500000, BigDecimal::from_str("0.08").unwrap()),
        Rebate::new(1000000, BigDecimal::from_str("0.01").unwrap()),
        Rebate::new(2500000,  BigDecimal::from_str("0.02").unwrap()),
        Rebate::new(7500000,BigDecimal::from_str("0.01").unwrap()),
        Rebate::new(10000000, BigDecimal::from_str("0.02").unwrap()),
    ];

    calculate_rebate_fn(total, Box::from(rebates))
}

fn calculate_rebate_test_fn(total: i32) -> BigDecimal {
    let rebates = [
        Rebate::new(500, BigDecimal::from_str("0.08").unwrap()),
        Rebate::new(1000, BigDecimal::from_str("0.01").unwrap()),
        Rebate::new(2500,  BigDecimal::from_str("0.02").unwrap()),
        Rebate::new(7500,BigDecimal::from_str("0.01").unwrap()),
        Rebate::new(10000, BigDecimal::from_str("0.02").unwrap()),
    ];

    calculate_rebate_fn(total, Box::from(rebates))
}

fn main() {
    println!("Jeg æder blåbærsyltetøj!");
    let envelopes = 1285540;
    let paper = 2583058;

    let test1 = 499;
    let test2 = 1000;
    let test3 = 9999;

    println!("{}", total_rebate_fn(envelopes));
    println!("{}", total_rebate_fn(paper));

    println!("Formatted some test to eight decimal precision.");
    assert_eq!(calculate_rebate_test_fn(test1), BigDecimal::from_i32(0).unwrap());
    assert_eq!(calculate_rebate_test_fn(test1 + 1), BigDecimal::from_f32(0.00016).unwrap());

    assert_eq!(format!("{0:.8}", calculate_rebate_test_fn(test2 - 1)), format!("{0:.8}", 0.04004004));
    assert_eq!(calculate_rebate_test_fn(test2), BigDecimal::from_f32(0.04009).unwrap());
    assert_eq!(format!("{0:.8}", calculate_rebate_test_fn(test2 + 1)), format!("{0:.8}", 0.04013986));

    assert_eq!(format!("{0:.8}", calculate_rebate_test_fn(test3)), format!("{0:.8}", 0.10251025));
    assert_eq!(calculate_rebate_test_fn(test3 + 1), BigDecimal::from_f32(0.102514).unwrap());
}
