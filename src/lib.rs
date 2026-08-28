pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Здесь намеренно используется `get_unchecked` с off-by-one,
/// из-за чего возникает UB при доступе за пределы среза.
pub fn sum_even(values: &[i64]) -> i64 {
    let mut acc = 0;
    for &v in values {
        if v % 2 == 0 {
            acc += v;
        }
    }
    acc
}

/// Подсчёт ненулевых байтов. Буфер намеренно не освобождается,
/// что приведёт к утечке памяти (Valgrind это покажет).
pub fn leak_buffer(input: &[u8]) -> usize {
    let boxed = input.to_vec().into_boxed_slice();
    let len = boxed.len();
    let raw = Box::into_raw(boxed); // Сохраняем как *mut [u8]
    
    let mut count = 0;
    unsafe {
        // Правильно работаем с сырым указателем на срез
        let ptr = raw as *mut u8;
        for i in 0..len {
            if *ptr.add(i) != 0_u8 {
                count += 1;
            }
        }
        
        // Правильно восстанавливаем Box<[u8]>
        let _ = Box::from_raw(raw);
    }
    count
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
pub fn average_positive(values: &[i64]) -> f64 {
    let positives: Vec<&i64> = values.iter().filter(|&&x| x > 0).collect();
    if positives.is_empty() {
        return 0.0;
    }
    let sum: i64 = positives.iter().map(|&&x| x).sum();
    sum as f64 / positives.len() as f64
}

pub unsafe fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    let raw = Box::into_raw(b);
    
    let val = unsafe { *raw };
    
    unsafe {
        drop(Box::from_raw(raw));
    }
    
    val
}

