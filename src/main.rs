#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
//
mod calculation;

use calculation::calculate_money_timeseries_after_months;
use eframe::egui;
use egui_plot::{Legend, Line, PlotPoints};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

struct Buyer {
    starting_money: f64,
    liquid_salary: f64,
    fixed_monthly_expenses: f64,
    investment_monthly_interest: f64,
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

struct House {
    house_price: f64,
    down_payment: f64,
    house_monthly_interest: f64,
    months_to_pay: i32,
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

struct Simulation {
    months_to_forecast: i32,
}

impl Default for Simulation {
    fn default() -> Self {
        Simulation {
            months_to_forecast: 120,
        }
    }
}

#[derive(Default)]
struct MyApp {
    buyer: Buyer,
    house: House,
    simulation: Simulation,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("House Buying");
            ui.label("Buyer params");
            ui.add(
                egui::Slider::new(&mut self.buyer.starting_money, 0.0..=2_000_000.0)
                    .text("Starting Money"),
            );
            ui.add(
                egui::Slider::new(&mut self.buyer.liquid_salary, 0.0..=100_000.0)
                    .text("Liquid Salary"),
            );
            ui.add(
                egui::Slider::new(&mut self.buyer.fixed_monthly_expenses, 0.0..=100_000.0)
                    .text("Fixed Monthly Expenses"),
            );
            ui.add(
                egui::Slider::new(&mut self.buyer.investment_monthly_interest, 0.0..=1.0)
                    .text("Investment Monthly Interest"),
            );
            ui.label("House Params");
            ui.add(
                egui::Slider::new(&mut self.house.house_price, 100_000.0..=2_000_000.0)
                    .text("House Price"),
            );
            ui.add(
                egui::Slider::new(&mut self.house.down_payment, 10_000.0..=2_000_000.0)
                    .text("Down Payment"),
            );
            ui.add(
                egui::Slider::new(&mut self.house.house_monthly_interest, 0.0..=1.0)
                    .text("House Monthly Interest"),
            );
            ui.add(egui::Slider::new(&mut self.house.months_to_pay, 1..=360).text("Months To Pay"));
            ui.label("Simulation");
            ui.add(
                egui::Slider::new(&mut self.simulation.months_to_forecast, 1..=720)
                    .text("Months To Simulate"),
            );

            let sim_output = calculate_money_timeseries_after_months(
                self.simulation.months_to_forecast,
                self.buyer.starting_money,
                self.house.down_payment,
                self.house.house_price,
                self.house.house_monthly_interest,
                self.house.months_to_pay,
                self.buyer.liquid_salary,
                self.buyer.fixed_monthly_expenses,
                self.buyer.investment_monthly_interest,
            );
            ui.label(format!(
                "Monthly payment: R$ {}",
                &sim_output.monthly_payment
            ));

            let points = PlotPoints::from_ys_f64(&sim_output.time_series);

            egui_plot::Plot::new("plot")
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(true)
                .legend(Legend::default())
                .show(ui, |plot_ui| {
                    plot_ui.line(Line::new("Money in Account", points))
                })
        });
    }
}
