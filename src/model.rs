pub(crate) struct Buyer {
    pub(crate) starting_money: f64,
    pub(crate) liquid_salary: f64,
    pub(crate) fixed_monthly_expenses: f64,
    pub(crate) investment_monthly_interest: f64,
}

impl Default for Buyer {
    fn default() -> Self {
        Buyer {
            starting_money: 600_000.0,
            liquid_salary: 20_000.0,
            fixed_monthly_expenses: 7_000.0,
            investment_monthly_interest: 0.01,
        }
    }
}

pub(crate) struct House {
    pub(crate) house_price: f64,
    pub(crate) down_payment: f64,
    pub(crate) house_monthly_interest: f64,
    pub(crate) months_to_pay: i32,
}

impl Default for House {
    fn default() -> Self {
        House {
            house_price: 600_000.0,
            down_payment: 150_000.0,
            house_monthly_interest: 0.01,
            months_to_pay: 120,
        }
    }
}

pub(crate) struct Simulation {
    pub(crate) months_to_forecast: i32,
}

impl Default for Simulation {
    fn default() -> Self {
        Simulation {
            months_to_forecast: 120,
        }
    }
}
