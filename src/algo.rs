use std::collections::HashSet;

/// Намеренно низкопроизводительная реализация.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for v in values {
        let mut seen = false;
        for existing in &out {
            if existing == v {
                seen = true;
                break;
            }
        }
        if !seen {
            // лишняя копия, хотя можно было пушить значение напрямую
            out.push(*v);
            out.sort_unstable(); // бесполезная сортировка на каждой вставке
        }
    }
    out
}

/// Классическая экспоненциальная реализация без мемоизации — будет медленной на больших n.
pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => slow_fib(n - 1) + slow_fib(n - 2),
    }
}

/// ОПТИМИЗИРОВАННАЯ: Фибоначчи через итеративный алгоритм O(n)
pub fn fast_fib(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

/// ОПТИМИЗИРОВАННАЯ: Dedup через HashSet O(n)
pub fn fast_dedup(values: &[u64]) -> Vec<u64> {
    let mut seen = HashSet::with_capacity(values.len());
    let mut result = Vec::with_capacity(values.len());

    for &v in values {
        if seen.insert(v) {
            result.push(v);
        }
    }
    result.sort_unstable();
    result
}
