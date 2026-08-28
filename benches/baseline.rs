use broken_app::algo::{slow_fib, slow_dedup, fast_fib, fast_dedup};
use broken_app::sum_even;
use std::time::Instant;

fn time_it(label: &str, mut f: impl FnMut()) {
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();
    println!("{label}: {:?}", elapsed);
}

fn main() {
    let data: Vec<i64> = (0..50_000).collect();
    let fib_n = 32;
    let dedup_data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();

    println!("=== ДО оптимизации (медленные версии) ===");
    for _ in 0..3 {
        time_it("sum_even", || {
            let _ = sum_even(&data);
        });
        time_it("slow_fib", || {
            let _ = slow_fib(fib_n);
        });
        time_it("slow_dedup", || {
            let _ = slow_dedup(&dedup_data);
        });
    }

    println!("\n=== ПОСЛЕ оптимизации (быстрые версии) ===");
    for _ in 0..3 {
        time_it("sum_even", || {
            let _ = sum_even(&data);
        });
        time_it("fast_fib", || {
            let _ = fast_fib(fib_n);
        });
        time_it("fast_dedup", || {
            let _ = fast_dedup(&dedup_data);
        });
    }

    println!("\n=== Проверка корректности ===");
    assert_eq!(slow_fib(20), fast_fib(20));
    assert_eq!(slow_dedup(&[5, 5, 1, 2, 2, 3]), fast_dedup(&[5, 5, 1, 2, 2, 3]));
    println!("Все тесты пройдены!");
}
