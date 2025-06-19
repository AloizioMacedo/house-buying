const ERR: f64 = 0.001;
const MAX_ITERS: i32 = 10_000;
const UPPER_BOUND: f64 = 100_000.0;

pub(crate) struct SimulationOutput {
    pub(crate) time_series: Vec<f64>,
    pub(crate) monthly_payment: f64,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn calculate_money_timeseries_after_months(
    months_to_forecast: i32,
    starting_money: f64,
    down_payment: f64,
    house_price: f64,
    house_monthly_interest: f64,
    n_months_to_pay: i32,
    liquid_salary: f64,
    fixed_monthly_expenses: f64,
    investment_monthly_interest: f64,
) -> SimulationOutput {
    let mut time_series: Vec<f64> = Vec::new();

    let mut money_left = starting_money - down_payment;
    time_series.push(money_left);

    let monthly_payment = calculate_monthly_payment(
        house_price - down_payment,
        house_monthly_interest,
        n_months_to_pay,
        ERR,
        MAX_ITERS,
        UPPER_BOUND,
    );

    for i in 0..months_to_forecast {
        if i <= n_months_to_pay {
            money_left -= monthly_payment;
        }

        money_left *= 1.0 + investment_monthly_interest;
        money_left += liquid_salary - fixed_monthly_expenses;

        time_series.push(money_left);
    }

    SimulationOutput {
        time_series,
        monthly_payment,
    }
}

fn calculate_monthly_payment(
    value: f64,
    monthly_interest: f64,
    n_months: i32,
    err: f64,
    max_iters: i32,
    upper_bound: f64,
) -> f64 {
    let mut a = 0.0;
    let mut b = upper_bound;
    let mut c = (a + b) / 2.0;

    for _ in 0..max_iters {
        let current_error = calculate_left(c, value, monthly_interest, n_months);

        if current_error.abs() < err {
            return c;
        }

        if current_error > 0.0 {
            a = c;
        } else {
            b = c;
        }

        c = (a + b) / 2.0;
    }

    c
}

fn calculate_left(monthly_payment: f64, total: f64, monthly_interest: f64, n_months: i32) -> f64 {
    let mut left = total * (1.0 + monthly_interest);

    for _ in 0..n_months {
        left -= monthly_payment;

        if left > 0.0 {
            left *= 1.0 + monthly_interest
        }
    }

    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_left() {
        // No interest, and no payment happening.
        assert_eq!(calculate_left(0.0, 400_000.0, 0.0, 10), 400_000.0);
        assert_eq!(calculate_left(0.0, 328_929.0, 0.0, 10), 328_929.0);

        // No interest, payments happening and finishing.
        assert_eq!(calculate_left(10_000.0, 300_000.0, 0.0, 30), 0.0);
        assert_eq!(calculate_left(20_000.0, 300_000.0, 0.0, 15), 0.0);

        // No interest, value left.
        assert_eq!(calculate_left(150.0, 1_000.0, 0.0, 5), 250.0);

        // No interest, overpayment.
        assert_eq!(calculate_left(300.0, 1_000.0, 0.0, 5), -500.0);

        // With interest
        // 1100 -> 800
        // 880 -> 580
        // 638 -> 338
        // 338 + 33.8 = 371.8
        assert!(calculate_left(300.0, 1_000.0, 0.1, 3) - 371.8 < 0.001);
    }

    #[test]
    fn test_calculate_monthly_payment() {
        assert!(
            (calculate_monthly_payment(600_000.0, 0.013, 60, ERR, MAX_ITERS, UPPER_BOUND)
                - 14_463.60)
                .abs()
                < 0.1
        );
        assert!(
            (calculate_monthly_payment(455_232.55, 0.0119, 52, ERR, MAX_ITERS, UPPER_BOUND)
                - 11_791.03)
                .abs()
                < 0.1
        );
        assert!(
            (calculate_monthly_payment(900_000.0, 0.0101, 240, ERR, MAX_ITERS, UPPER_BOUND)
                - 9_985.17)
                .abs()
                < 0.1
        );
    }
}
