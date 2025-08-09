use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
    active: bool,
}

pub fn run() -> Result<()> {
    // Пример JSON-строки
    let data = r#"
        [
            { "id": 1, "name": "Alice", "email": "alice@example.com", "active": true },
            { "id": 2, "name": "Bob", "email": "bob@example.com", "active": false },
            { "id": 3, "name": "Charlie", "email": "charlie@example.com", "active": true }
        ]
    "#;

    // Парсим JSON в вектор структур
    let users: Vec<User> = serde_json::from_str(data)?;

    println!("All users:");
    for user in &users {
        println!("{:?}", user);
    }

    // Фильтрация активных пользователей
    let active_users: Vec<&User> = users.iter().filter(|u| u.active).collect();

    println!("\nActive users:");
    for user in active_users {
        println!("{} ({})", user.name, user.email);
    }

    // Динамический доступ к данным (через Value)
    let v: Value = serde_json::from_str(data)?;
    println!("\nName of first user: {}", v[0]["name"]);

    Ok(())
}
