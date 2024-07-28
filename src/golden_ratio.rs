use bigdecimal::BigDecimal;

fn derivative(n: BigDecimal) -> BigDecimal {
    BigDecimal::from(2) * n - BigDecimal::from(1)
}

fn golden_ratio(n: BigDecimal) -> BigDecimal {
    n.clone() * n.clone() - n - BigDecimal::from(1)
}

pub(crate) fn approximate() {
    let mut phi: BigDecimal = BigDecimal::from(1);
    for i in 0..1006 {
        print!("Iteration: {}\r", i + 1);
        phi = phi.clone() - golden_ratio(phi.clone()) / derivative(phi.clone());
    }
    println!("{}", phi);
}