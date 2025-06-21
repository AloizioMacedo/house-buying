use egui::{Color32, Grid, Ui};
use egui_plot::{Legend, Line, PlotPoints};

use crate::{
    AmortizationStrategyType, PlotSelection,
    calculation::SimulationOutput,
    format_with_thousands_separator,
    model::{Buyer, House, Simulation},
    plotting,
};

pub(crate) fn render_buyer_params(ui: &mut Ui, buyer: &mut Buyer) {
    ui.heading("Buyer params");
    ui.add(egui::Slider::new(&mut buyer.starting_money, 0.0..=2_000_000.0).text("Starting Money"));
    ui.add(egui::Slider::new(&mut buyer.liquid_salary, 0.0..=100_000.0).text("Liquid Salary"));
    ui.add(
        egui::Slider::new(&mut buyer.fixed_monthly_expenses, 0.0..=100_000.0)
            .text("Fixed Monthly Expenses"),
    );
    ui.add(
        egui::Slider::new(&mut buyer.investment_monthly_interest, 0.0..=1.0)
            .text("Investment Monthly Interest"),
    );
    ui.add(egui::Slider::new(&mut buyer.yearly_bonus, 0.0..=2_000_000.0).text("Yearly Bonus"));
}

pub(crate) fn render_house_params(
    ui: &mut Ui,
    house: &mut House,
    strategy: AmortizationStrategyType,
) {
    ui.heading("House Params");
    ui.horizontal(|ui| {
        ui.add(egui::Slider::new(&mut house.house_price, 0.0..=2_000_000.0).text("House Price"));
        if matches!(strategy, AmortizationStrategyType::Sac) {
            ui.add(
                egui::Slider::new(&mut house.yearly_extra_amortization, 0.0..=2_000_000.0)
                    .text("Yearly Extra Amortization"),
            );
        }
    });

    ui.add(egui::Slider::new(&mut house.down_payment, 0.0..=2_000_000.0).text("Down Payment"));
    ui.add(
        egui::Slider::new(&mut house.house_monthly_interest, 0.0..=1.0)
            .text("House Monthly Interest"),
    );
    ui.add(egui::Slider::new(&mut house.months_to_pay, 1..=360).text("Months To Pay"));
}

pub(crate) fn render_simulation_params(
    ui: &mut Ui,
    simulation: &mut Simulation,
    strategy: &mut AmortizationStrategyType,
    plot_selection: &mut PlotSelection,
) {
    ui.heading("Simulation");
    ui.add(
        egui::Slider::new(&mut simulation.months_to_forecast, 1..=720).text("Months To Simulate"),
    );
    ui.add(egui::Slider::new(&mut simulation.inflation, 0.0..=1.0).text("Inflation"));

    ui.horizontal(|ui| {
        ui.selectable_value(strategy, AmortizationStrategyType::Sac, "Tabela SAC");
        ui.selectable_value(strategy, AmortizationStrategyType::Price, "Tabela PRICE");
    });

    ui.horizontal(|ui| {
        ui.selectable_value(
            plot_selection,
            PlotSelection::MoneyInAccount,
            "Money In Account",
        );
        ui.selectable_value(plot_selection, PlotSelection::Payments, "Payments");
    });
}

pub(crate) fn render_kpis(
    ui: &mut Ui,
    strategy: AmortizationStrategyType,
    sim_output: &SimulationOutput,
    simulation: &Simulation,
) {
    Grid::new("grid").show(ui, |ui| {
        ui.label("Initial Money:");
        ui.label(format!(
            "R$ {}",
            format_with_thousands_separator(sim_output.time_series[0])
        ));
        ui.end_row();

        ui.label("Monthly Payment:");
        match strategy {
            AmortizationStrategyType::Sac => {
                if !sim_output.monthly_payments.is_empty() {
                    ui.label(format!(
                        "First: R$ {};",
                        format_with_thousands_separator(sim_output.monthly_payments[0])
                    ));
                } else {
                    ui.label("First: R$ NaN");
                }
                match sim_output.monthly_payments.last() {
                    Some(v) => {
                        ui.label(format!("Last: R$ {};", format_with_thousands_separator(*v)));
                    }
                    None => {
                        ui.label("Last: R$ NaN");
                    }
                }
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
            simulation.months_to_forecast
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
}

pub(crate) fn render_plot(
    ui: &mut Ui,
    sim_output: &SimulationOutput,
    plot_selection: PlotSelection,
) {
    match plot_selection {
        PlotSelection::MoneyInAccount => {
            let money_in_account = PlotPoints::from_ys_f64(&sim_output.time_series);

            egui_plot::Plot::new("plot")
                .y_axis_formatter(plotting::format_y_axis)
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(true)
                .legend(Legend::default())
                .show(ui, |plot_ui| {
                    plot_ui.line(
                        Line::new("Money in Account", money_in_account).color(Color32::DARK_GREEN),
                    )
                });
        }
        PlotSelection::Payments => {
            let payments = PlotPoints::from_ys_f64(&sim_output.monthly_payments);

            egui_plot::Plot::new("plot")
                .y_axis_formatter(plotting::format_y_axis)
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(true)
                .legend(Legend::default())
                .show(ui, |plot_ui| plot_ui.line(Line::new("Payments", payments)));
        }
    }
}
