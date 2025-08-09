use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Record {
    product: String,
    price: f64,
    quantity: u32,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // Открываем CSV-файл
    let file = File::open("data.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut total_revenue = 0.0;
    let mut total_price = 0.0;
    let mut count = 0;

    for result in rdr.deserialize() {
        let record: Record = result?;
        let revenue = record.price * record.quantity as f64;
        total_revenue += revenue;
        total_price += record.price;
        count += 1;
        println!(
            "Product: {}, Price: {}, Quantity: {}, Revenue: {}",
            record.product, record.price, record.quantity, revenue
        );
    }

    let avg_price = if count > 0 { total_price / count as f64 } else { 0.0 };

    println!("===================");
    println!("Total revenue: {:.2}", total_revenue);
    println!("Average price: {:.2}", avg_price);

    Ok(())
}
