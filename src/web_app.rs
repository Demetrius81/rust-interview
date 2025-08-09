use reqwest::Error;

pub async fn run() -> Result<(), Error> {
    // Делаем GET-запрос
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let response = reqwest::get(url).await?;

    // Читаем тело как строку
    let body = response.text().await?;

    println!("Ответ сервера:\n{}", body);

    Ok(())
}