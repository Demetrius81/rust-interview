use std::error::Error;
use std::fmt;

// Определим свой тип ошибки
#[derive(Debug)]
struct InvalidNumberError(String);

impl fmt::Display for InvalidNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid number: {}", self.0)
    }
}

impl Error for InvalidNumberError {}

// Функция для поиска НОД
fn gcd(a: i64, b: i64) -> Result<u64, InvalidNumberError> {
    if a <= 0 || b <= 0 {
        return Err(InvalidNumberError(format!("all numbers must be positive")));
    }

    let mut x = a as u64;
    let mut y = b as u64;

    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }

    Ok(x)
}

pub fn run() -> Result<(), Box<dyn Error>> {
    match gcd(48, 18) {
        Ok(result) => println!("GCD = {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }

    match gcd(48, 0) {
        Ok(result) => println!("GCD = {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}