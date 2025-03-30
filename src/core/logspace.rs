pub fn logspace(start: i32, end: i32, count: usize) -> Vec<i32> {
    let log_start = (start as f64).ln();
    let log_end = (end as f64).ln();
    (0..count)
        .map(|i| ((log_start + (i as f64 * (log_end - log_start) / (count as f64 - 1.0))).exp()) as i32)
        .collect()
}
