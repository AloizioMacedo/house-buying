import pandas as pd
import plotly.graph_objects as go

from house_buying.calculations import (
    SimulationOutput,
    calculate_money_timeseries_after_months,
)
from house_buying.config import load_config
from pathlib import Path


CONFIG_PATH = Path(__file__).parent.joinpath("config", "config.toml")
OUTPUT_PATH = Path(__file__).parent.joinpath("data", "outputs")


def plot_and_save_timeseries(
    simulation_output: SimulationOutput, name: str = "output.html"
):
    ser = pd.Series(simulation_output.time_series)

    initial_money = ser.iloc[0]
    money_after_one_year = ser.iloc[min(len(ser) - 1, 4 * 12 - 1)]
    money_after_five_years = ser.iloc[min(len(ser) - 1, 12 - 1)]

    info_text = (
        f"<b>Initial Money:</b> R${initial_money:,.2f}<br>"
        f"<b>Monthly Payment:</b> R${simulation_output.monthly_payment:,.2f}<br>"
        f"<b>Money After 1 Year:</b> R${money_after_one_year:,.2f}<br>"
        f"<b>Money After 5 Years:</b> R${money_after_five_years:,.2f}"
    )

    fig = go.Figure()

    fig.add_trace(
        go.Scatter(
            x=ser.index,
            y=ser.values,
            mode="lines+markers",
            name="Money in Account",
            hovertemplate="Month: %{x}<br>Amount: R$%{y:,.2f}<extra></extra>",
        )
    )

    # Add annotation box on the right
    fig.add_annotation(
        text=info_text,
        xref="paper",
        yref="paper",
        x=1.05,
        y=0.5,
        showarrow=False,
        align="left",
        bordercolor="black",
        borderwidth=1,
        bgcolor="white",
        font=dict(size=12),
    )

    fig.update_layout(
        title="Money in Account",
        xaxis_title="Date",
        yaxis_title="Amount (R$)",
        yaxis_tickprefix="R$",
        yaxis_tickformat=", .2f",
        template="plotly_white",
        autosize=True,
        margin=dict(r=150),
    )

    OUTPUT_PATH.mkdir(parents=True, exist_ok=True)
    fig.write_html(OUTPUT_PATH.joinpath(name))


def main():
    config = load_config(str(CONFIG_PATH))

    down_payments = (
        config.house.down_payment
        if isinstance(config.house.down_payment, list)
        else [float(config.house.down_payment)]
    )
    house_prices = (
        config.house.house_price
        if isinstance(config.house.house_price, list)
        else [float(config.house.house_price)]
    )
    months_to_pay = (
        config.house.months_to_pay
        if isinstance(config.house.months_to_pay, list)
        else [int(config.house.months_to_pay)]
    )

    for down_payment in down_payments:
        for house_price in house_prices:
            for months in months_to_pay:
                output = calculate_money_timeseries_after_months(
                    config.simulation.months_to_forecast,
                    config.buyer.starting_money,
                    down_payment,
                    house_price,
                    config.house.house_monthly_interest,
                    months,
                    config.buyer.money_saved_monthly,
                    config.buyer.investment_monthly_interest,
                )

                plot_and_save_timeseries(
                    output, f"{int(house_price)}P_{int(down_payment)}D_{months}M.html"
                )


if __name__ == "__main__":
    main()
