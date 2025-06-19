pub(crate) fn format_y_axis(
    grid_mark: egui_plot::GridMark,
    _range: &std::ops::RangeInclusive<f64>,
) -> String {
    let abs = grid_mark.value.abs() as u64;
    let sign = if grid_mark.value < 0.0 { "-" } else { "" };
    let formatted = format_with_thousands_separator(abs as f64);

    format!("{}R$ {}", sign, formatted)
}

pub(crate) fn format_with_thousands_separator(num: f64) -> String {
    let integer_part = num.trunc();
    let int_str = integer_part.to_string();
    let mut chars = int_str.chars().rev().collect::<Vec<_>>();
    let mut parts = Vec::new();

    while !chars.is_empty() {
        let chunk: String = chars.drain(..chars.len().min(3)).collect();
        parts.push(chunk.chars().rev().collect::<String>());
    }

    let decimal_part = num.fract();
    let decimal_part_str = format!("{:.2}", decimal_part).to_string();

    parts.into_iter().rev().collect::<Vec<_>>().join(",") + &decimal_part_str[1..]
}

#[cfg(test)]
mod tests {
    use super::format_with_thousands_separator;

    #[test]
    fn test_formatting() {
        let number = 5523.1238;
        assert_eq!(format_with_thousands_separator(number), "5,523.12");

        let number = 328.0;
        assert_eq!(format_with_thousands_separator(number), "328.00");

        let number = 0.64;
        assert_eq!(format_with_thousands_separator(number), "0.64");

        let number = 1000.120120120;
        assert_eq!(format_with_thousands_separator(number), "1,000.12");

        let number = 1_200_300.1;
        assert_eq!(format_with_thousands_separator(number), "1,200,300.10");
    }
}
