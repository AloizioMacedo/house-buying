use egui::{Color32, Grid, Ui};
use egui_plot::{Legend, Line, PlotPoints};

use crate::{
    calculation::SimulationOutput,
    format_with_thousands_separator,
    model::{Buyer, House, Simulation},
    plotting, AmortizationStrategyType, PlotSelection,
};

pub(crate) fn render_buyer_params(ui: &mut Ui, buyer: &mut Buyer) {
    ui.heading("Parâmetros do Comprador");
    ui.add(
        egui::Slider::new(&mut buyer.starting_money, 0.0..=2_000_000.0).text("Dinheiro Inicial"),
    );
    ui.add(egui::Slider::new(&mut buyer.liquid_salary, 0.0..=100_000.0).text("Salário Líquido"));
    ui.add(
        egui::Slider::new(&mut buyer.fixed_monthly_expenses, 0.0..=100_000.0)
            .text("Gastos Mensais"),
    );
    ui.add(egui::Slider::new(&mut buyer.yearly_bonus, 0.0..=2_000_000.0).text("Bônus Anual"));
    ui.add(
        egui::Slider::new(&mut buyer.investment_monthly_interest, 0.0..=1.0)
            .text("Taxa de Lucro em Investimentos"),
    );
}

pub(crate) fn render_house_params(
    ui: &mut Ui,
    house: &mut House,
    strategy: AmortizationStrategyType,
) {
    ui.heading("Parâmetros do Financiamento");
    ui.add(
        egui::Slider::new(&mut house.house_price, 0.0..=2_000_000.0).text("Preço Total da Casa"),
    );
    ui.add(egui::Slider::new(&mut house.down_payment, 0.0..=2_000_000.0).text("Entrada"));
    ui.add(egui::Slider::new(&mut house.months_to_pay, 1..=360).text("Número de Parcelas"));
    ui.add_enabled(
        matches!(strategy, AmortizationStrategyType::Sac),
        egui::Slider::new(&mut house.yearly_extra_amortization, 0.0..=2_000_000.0)
            .text("Amortização Extra Anual"),
    );
    ui.add(egui::Slider::new(&mut house.house_monthly_interest, 0.0..=1.0).text("Juros Mensal"));
}

pub(crate) fn render_simulation_params(
    ui: &mut Ui,
    simulation: &mut Simulation,
    strategy: &mut AmortizationStrategyType,
    plot_selection: &mut PlotSelection,
) {
    ui.heading("Simulação");
    ui.add(
        egui::Slider::new(&mut simulation.months_to_forecast, 1..=720).text("Meses para Simular"),
    );
    ui.add(egui::Slider::new(&mut simulation.inflation, 0.0..=1.0).text("Inflação"));

    Grid::new("buttons").show(ui, |ui| {
        ui.label("Tabela:");
        ui.selectable_value(strategy, AmortizationStrategyType::Sac, "Tabela SAC");
        ui.selectable_value(strategy, AmortizationStrategyType::Price, "Tabela PRICE");
        ui.end_row();
        ui.label("Plot:");
        ui.selectable_value(
            plot_selection,
            PlotSelection::MoneyInAccount,
            "Dinheiro na Conta",
        );
        ui.selectable_value(plot_selection, PlotSelection::Payments, "Pagamentos");
        ui.end_row();
    });
}

pub(crate) fn render_kpis(ui: &mut Ui, sim_output: &SimulationOutput, simulation: &Simulation) {
    Grid::new("grid").show(ui, |ui| {
        ui.label("Dinheiro Inicial:");
        ui.label(format!(
            "{}",
            format_with_thousands_separator(sim_output.time_series[0])
        ));
        ui.end_row();

        ui.label("Parcelas Mensais");
        ui.label(format!(
            "Primeira: {}",
            format_with_thousands_separator(
                sim_output.monthly_payments.first().copied().unwrap_or(0.0)
            )
        ));
        ui.end_row();
        ui.label("");
        ui.label(format!(
            "Última: {}",
            format_with_thousands_separator(
                sim_output.monthly_payments.last().copied().unwrap_or(0.0)
            )
        ));
        ui.end_row();

        ui.label("Parcels terminam em:");
        ui.label(format!("{} meses", sim_output.ends_after));
        ui.end_row();

        ui.label("Dinheiro depois de 1 ano:");
        match sim_output.time_series.get(12 - 1) {
            Some(v) => {
                ui.label(format!("{}", format_with_thousands_separator(*v)));
            }
            None => {
                ui.label("NaN");
            }
        }
        ui.end_row();

        ui.label("Dinheiro depois de 5 anos:");
        match sim_output.time_series.get(5 * 12 - 1) {
            Some(v) => {
                ui.label(format!("{}", format_with_thousands_separator(*v)));
            }
            None => {
                ui.label("NaN");
            }
        }
        ui.end_row();

        ui.label(format!(
            "Dinheiro no fim da sim ({} meses)",
            simulation.months_to_forecast
        ));
        match sim_output.time_series.last() {
            Some(v) => {
                ui.label(format!("{}", format_with_thousands_separator(*v)));
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
                        Line::new("Dinheiro na Conta", money_in_account).color(Color32::DARK_GREEN),
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
                .show(ui, |plot_ui| {
                    plot_ui.line(Line::new("Pagamentos", payments))
                });
        }
    }
}
