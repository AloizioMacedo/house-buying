#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod calculation;
mod model;
mod plotting;
mod ui_components;

use calculation::{
    calculate_money_timeseries_price, calculate_money_timeseries_sac, AmortizationStrategyType,
};
use eframe::egui;
use plotting::format_with_thousands_separator;
use ui_components::{
    render_buyer_params, render_house_params, render_kpis, render_plot, render_simulation_params,
};

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_| Ok(Box::<MyApp>::default())),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    })
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
            ui.style_mut().spacing.slider_width = 120.0;

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
                    self.simulation.inflation,
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
                    self.simulation.inflation,
                ),
            };

            render_kpis(ui, self.strategy, &sim_output, &self.simulation);
            render_plot(ui, &sim_output, self.plot_selection);
        });
    }
}
