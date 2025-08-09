use std::time::Instant;
use tokio::task;

/// Считаем сумму чисел
fn sum_of_nums(a: u64, b: u64) -> u64 {
    a + b
}

pub async fn run() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let start_time = Instant::now();

    // Запускаем задачи параллельно
    let mut handles = Vec::new();
    for num in arr {
        handles.push(task::spawn_blocking(move || sum_of_nums(num, num)));
    }

    // Ждём результатов
    let mut total: u64 = 0;
    for handle in handles {
        total += handle.await.expect("--->task failed");
    }

    let elapsed = start_time.elapsed();
    println!("--->Total sum of nums: {}", total);
    println!("--->Elapsed: {:.2?}", elapsed);
}
