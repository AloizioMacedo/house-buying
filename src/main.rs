#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod calculation;
mod model;
mod plotting;
mod ui_components;

use calculation::{
    AmortizationStrategyType, calculate_money_timeseries_price, calculate_money_timeseries_sac,
};
use eframe::egui;
use plotting::format_with_thousands_separator;
use ui_components::{
    render_buyer_params, render_house_params, render_kpis, render_plot, render_simulation_params,
};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size((1024.0, 768.0))
            .with_title("House Buying")
            .with_resizable(true)
            .with_visible(true)
            .with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "House Buying",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
enum PlotSelection {
    #[default]
    MoneyInAccount,
    Payments,
}

#[derive(Default)]
struct MyApp {
    buyer: model::Buyer,
    house: model::House,
    simulation: model::Simulation,

    strategy: calculation::AmortizationStrategyType,
    plot_selection: PlotSelection,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            render_buyer_params(ui, &mut self.buyer);
            render_house_params(ui, &mut self.house, self.strategy);
            render_simulation_params(
                ui,
                &mut self.simulation,
                &mut self.strategy,
                &mut self.plot_selection,
            );

            let sim_output = match self.strategy {
                AmortizationStrategyType::Price => calculate_money_timeseries_price(
                    self.simulation.months_to_forecast,
                    self.buyer.starting_money,
                    self.house.down_payment,
                    self.house.house_price,
                    self.house.house_monthly_interest,
                    self.house.months_to_pay,
                    self.buyer.liquid_salary,
                    self.buyer.fixed_monthly_expenses,
                    self.buyer.investment_monthly_interest,
                    self.buyer.yearly_bonus,
                ),
                AmortizationStrategyType::Sac => calculate_money_timeseries_sac(
                    self.simulation.months_to_forecast,
                    self.buyer.starting_money,
                    self.house.down_payment,
                    self.house.house_price,
                    self.house.house_monthly_interest,
                    self.house.months_to_pay,
                    self.buyer.liquid_salary,
                    self.buyer.fixed_monthly_expenses,
                    self.buyer.investment_monthly_interest,
                    self.buyer.yearly_bonus,
                    self.house.yearly_extra_amortization,
                ),
            };

            render_kpis(ui, self.strategy, &sim_output, &self.simulation);
            render_plot(ui, &sim_output, self.plot_selection);
        });
    }
}
