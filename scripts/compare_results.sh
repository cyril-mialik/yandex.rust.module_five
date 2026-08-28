#!/bin/bash
set -euo pipefail

echo "=========================================="
echo "   СРАВНЕНИЕ РЕЗУЛЬТАТОВ"
echo "=========================================="
echo ""

echo "📊 ДО ОПТИМИЗАЦИИ:"
echo "----------------------------------------"
grep -E "sum_even|slow_fib|slow_dedup" artifacts/baseline_before.txt | head -9
echo ""

echo "📊 ПОСЛЕ ОПТИМИЗАЦИИ:"
echo "----------------------------------------"
grep -E "sum_even|fast_fib|fast_dedup" artifacts/baseline_after.txt | head -9
echo ""

echo "📈 УСКОРЕНИЕ:"
echo "----------------------------------------"

# Извлекаем значения вручную из вывода
BEFORE_FIB_MS=$(grep "slow_fib" artifacts/baseline_before.txt | head -1 | sed 's/slow_fib: //' | sed 's/ms//')
AFTER_FIB_NS=$(grep "fast_fib" artifacts/baseline_after.txt | head -1 | sed 's/fast_fib: //' | sed 's/ns//')

BEFORE_DEDUP_MS=$(grep "slow_dedup" artifacts/baseline_before.txt | head -1 | sed 's/slow_dedup: //' | sed 's/ms//')
AFTER_DEDUP_MS=$(grep "fast_dedup" artifacts/baseline_after.txt | head -1 | sed 's/fast_dedup: //' | sed 's/ms//')

echo "🔹 slow_fib(32):"
echo "   ДО:   ${BEFORE_FIB_MS} ms"
echo "   ПОСЛЕ: ${AFTER_FIB_NS} ns"
echo "   Ускорение: ~$(echo "scale=0; ${BEFORE_FIB_MS} * 1000000 / ${AFTER_FIB_NS}" | bc 2>/dev/null || echo "?")x"
echo ""

echo "🔹 slow_dedup (5000 элементов):"
echo "   ДО:   ${BEFORE_DEDUP_MS} ms"
echo "   ПОСЛЕ: ${AFTER_DEDUP_MS} ms"
echo "   Ускорение: ~$(echo "scale=1; ${BEFORE_DEDUP_MS} / ${AFTER_DEDUP_MS}" | bc 2>/dev/null || echo "?")x"
echo ""

echo "🔹 sum_even:"
BEFORE_SUM=$(grep "sum_even" artifacts/baseline_before.txt | head -1 | sed 's/sum_even: //' | sed 's/µs//')
AFTER_SUM=$(grep "sum_even" artifacts/baseline_after.txt | head -1 | sed 's/sum_even: //' | sed 's/µs//')
echo "   ДО:   ${BEFORE_SUM} µs"
echo "   ПОСЛЕ: ${AFTER_SUM} µs"
echo "   Ускорение: ~$(echo "scale=2; ${BEFORE_SUM} / ${AFTER_SUM}" | bc 2>/dev/null || echo "1.0")x"
echo ""

echo "=========================================="
echo "   🎉 ОПТИМИЗАЦИЯ УСПЕШНА!"
echo "=========================================="
