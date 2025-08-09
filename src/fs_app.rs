use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    // Создаём или открываем файл для записи
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("example.txt")?;

    // Записываем данные в файл
    writeln!(file, "Hello, Rust filesystem!")?;

    // Открываем файл для чтения
    let mut file = File::open("example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Содержимое файла:\n{}", contents);

    Ok(())
}
