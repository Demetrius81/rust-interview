use rusqlite::{params, Connection, Result};

pub fn run() -> Result<()> {
    // Подключение к базе данных (создаст файл, если его нет)
    let conn = Connection::open("users.db")?;

    // Создаём таблицу
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )",
        [],
    )?;

    // Добавляем пользователя
    conn.execute(
        "INSERT INTO user (name, age) VALUES (?1, ?2)",
        params!["Alice", 30],
    )?;

    // Читаем всех пользователей
    let mut stmt = conn.prepare("SELECT id, name, age FROM user")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?, // id
            row.get::<_, String>(1)?, // name
            row.get::<_, i32>(2)?, // age
        ))
    })?;

    println!("Users in DB:");
    for user in rows {
        let (id, name, age) = user?;
        println!("ID: {}, Name: {}, Age: {}", id, name, age);
    }

    Ok(())
}
