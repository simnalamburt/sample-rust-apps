use std::{error::Error, io::stdin};

use csv::Reader;
use fastnum::{UD128, decimal::Context};

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_reader(stdin().lock());

    let mut count = 0;
    let mut sum = UD128::ZERO;
    let context = Context::default();

    for row in reader.records() {
        let cell = &row?[6];
        let revenue = UD128::from_str(cell, context)?;

        count += 1;
        sum += revenue;
    }

    if !sum.is_op_ok() {
        return Err("Numerical error occurred".into());
    }

    println!("count: {count}\nsum: {sum}");

    Ok(())
}
