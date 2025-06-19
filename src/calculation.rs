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
