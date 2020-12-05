use bigdecimal::{BigDecimal, FromPrimitive};

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

fn total_rebate_fn(total: i32) -> BigDecimal {
    let rebates = [
        Rebate::new(500000, BigDecimal::from_f32(0.08).unwrap()),
        Rebate::new(1000000, BigDecimal::from_f32(0.01).unwrap()),
        Rebate::new(2500000,  BigDecimal::from_f32(0.02).unwrap()),
        Rebate::new(7500000,BigDecimal::from_f32(0.01).unwrap()),
        Rebate::new(10000000, BigDecimal::from_f32(0.02).unwrap()),
    ];

    let mut total_rebate = BigDecimal::from_i32(0).unwrap();

    let mut rebate = rebates.iter();

    loop {
        match rebate.next() {
            Some(r) => {
                if r.volume > total {
                    break
                } else {
                    let remaining = total - &r.volume;
                    total_rebate += BigDecimal::from_f32(remaining as f32 / total as f32).unwrap() * &r.rebate;
                }
            }
            None => break,
        }
    }
    total_rebate
}

fn main() {
    println!("Jeg æder blåbærsyltetøj!");
    let envelopes = 1285540;
    let paper = 2583058;
    let test = 1000000;

    println!("{}", total_rebate_fn(envelopes));
    println!("{}", total_rebate_fn(paper));
    println!("{}", total_rebate_fn(test));
}
