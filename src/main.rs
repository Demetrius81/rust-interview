mod data_app;
mod db_app;
mod fs_app;
mod gcd_mod;
mod json_parser;
mod parallel_app;
mod prime_nums;
mod quick_sort;
mod stack_linked_list;
mod stack_mod;
mod web_app;

#[tokio::main]
async fn main() {
    gcd_mod::run().unwrap_or_else(|e| {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    });

    stack_mod::run();

    stack_linked_list::run();

    prime_nums::run();

    quick_sort::run();

    _ = json_parser::run();

    parallel_app::run().await;

    data_app::run().unwrap();

    db_app::run().unwrap();

    _ = web_app::run().await;

    _ = fs_app::run();
}
