mod gcd_mod;
mod prime_nums;
mod stack_linked_list;
mod stack_mod;

fn main() {
    gcd_mod::run().unwrap_or_else(|e| {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    });

    stack_mod::run();

    stack_linked_list::run();

    prime_nums::run();
}
