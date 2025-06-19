#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod calculation;
mod model;
mod plotting;

use calculation::{
    AmortizationStrategyType, calculate_money_timeseries_price, calculate_money_timeseries_sac,
};
use eframe::egui;
use egui::Grid;
use egui_plot::{Legend, Line, PlotPoints};
use plotting::format_with_thousands_separator;

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

#[derive(Default)]
struct MyApp {
    buyer: model::Buyer,
    house: model::House,
    simulation: model::Simulation,

    strategy: calculation::AmortizationStrategyType,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Buyer params");
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
            ui.add(
                egui::Slider::new(&mut self.buyer.yearly_bonus, 0.0..=2_000_000.0)
                    .text("Yearly Bonus"),
            );
            ui.heading("House Params");
            ui.horizontal(|ui| {
                ui.add(
                    egui::Slider::new(&mut self.house.house_price, 0.0..=2_000_000.0)
                        .text("House Price"),
                );
                if matches!(self.strategy, AmortizationStrategyType::Sac) {
                    ui.add(
                        egui::Slider::new(
                            &mut self.house.yearly_extra_amortization,
                            0.0..=2_000_000.0,
                        )
                        .text("Yearly Extra Amortization"),
                    );
                }
            });

            ui.add(
                egui::Slider::new(&mut self.house.down_payment, 0.0..=2_000_000.0)
                    .text("Down Payment"),
            );
            ui.add(
                egui::Slider::new(&mut self.house.house_monthly_interest, 0.0..=1.0)
                    .text("House Monthly Interest"),
            );
            ui.add(egui::Slider::new(&mut self.house.months_to_pay, 1..=360).text("Months To Pay"));
            ui.heading("Simulation");
            ui.add(
                egui::Slider::new(&mut self.simulation.months_to_forecast, 1..=720)
                    .text("Months To Simulate"),
            );

            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.strategy,
                    AmortizationStrategyType::Sac,
                    "Tabela SAC",
                );
                ui.selectable_value(
                    &mut self.strategy,
                    AmortizationStrategyType::Price,
                    "Tabela PRICE",
                );
            });

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

            Grid::new("grid").show(ui, |ui| {
                ui.label("Initial Money:");
                ui.label(format!(
                    "R$ {}",
                    format_with_thousands_separator(sim_output.time_series[0])
                ));
                ui.end_row();

                ui.label("Monthly Payment:");
                match self.strategy {
                    AmortizationStrategyType::Sac => {
                        ui.label(format!(
                            "First: R$ {};",
                            format_with_thousands_separator(sim_output.monthly_payments[0])
                        ));
                        if sim_output.monthly_payments.len() >= 12 {
                            ui.label(format!(
                                "One Year: R$ {};",
                                format_with_thousands_separator(sim_output.monthly_payments[11])
                            ));
                        } else {
                            ui.label("One Year: R$ NaN");
                        }
                        if sim_output.monthly_payments.len() >= 60 {
                            ui.label(format!(
                                "Five Years: R$ {};",
                                format_with_thousands_separator(
                                    sim_output.monthly_payments[60 - 1]
                                )
                            ));
                        } else {
                            ui.label("Five Years: R$ NaN;");
                        }
                        ui.label(format!(
                            "Last: R$ {};",
                            format_with_thousands_separator(
                                *sim_output
                                    .monthly_payments
                                    .last()
                                    .expect("should have at least one monthly payment")
                            )
                        ));
                        ui.label(format!("Ends after {} months", sim_output.ends_after));
                    }
                    AmortizationStrategyType::Price => {
                        ui.label(format!(
                            "R$ {}",
                            format_with_thousands_separator(sim_output.monthly_payments[0])
                        ));
                    }
                }
                ui.end_row();

                ui.label("Money After 1 Year:");
                match sim_output.time_series.get(12 - 1) {
                    Some(v) => {
                        ui.label(format!("R$ {}", format_with_thousands_separator(*v)));
                    }
                    None => {
                        ui.label("NaN");
                    }
                }
                ui.end_row();

                ui.label("Money After 5 Years:");
                match sim_output.time_series.get(5 * 12 - 1) {
                    Some(v) => {
                        ui.label(format!("R$ {}", format_with_thousands_separator(*v)));
                    }
                    None => {
                        ui.label("NaN");
                    }
                }
                ui.end_row();

                ui.label(format!(
                    "Money at End of Sim ({} months)",
                    self.simulation.months_to_forecast
                ));
                match sim_output.time_series.last() {
                    Some(v) => {
                        ui.label(format!("R$ {}", format_with_thousands_separator(*v)));
                    }
                    None => {
                        ui.label("NaN");
                    }
                }

                ui.end_row();
            });

            let points = PlotPoints::from_ys_f64(&sim_output.time_series);

            egui_plot::Plot::new("plot")
                .y_axis_formatter(plotting::format_y_axis)
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
