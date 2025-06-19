#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
//
mod calculation;

use eframe::egui;
use egui::RichText;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
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

struct MyApp {
    name: String,
    age: u32,

    buyer: Buyer,
    house: House,
    simulation: Simulation,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            buyer: Buyer::default(),
            house: House::default(),
            simulation: Simulation::default(),
        }
    }
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
                    .text("Months To Pay"),
            );
        });
    }
}
