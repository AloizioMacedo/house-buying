pub(crate) fn format_y_axis(
    grid_mark: egui_plot::GridMark,
    _range: &std::ops::RangeInclusive<f64>,
) -> String {
    let abs = grid_mark.value.abs() as u64;
    let sign = if grid_mark.value < 0.0 { "-" } else { "" };
    let formatted = format!("{abs}");

    // Insert dots as thousand separators
    let mut parts = Vec::new();
    let mut chars = formatted.chars().rev().collect::<Vec<_>>();
    while !chars.is_empty() {
        let chunk: String = chars.drain(..chars.len().min(3)).collect();
        parts.push(chunk);
    }
    let with_dots = parts
        .into_iter()
        .map(|s| s.chars().rev().collect::<String>())
        .rev()
        .collect::<Vec<_>>()
        .join(".");

    format!("{}${}", sign, with_dots)
}
