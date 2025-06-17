import pandas as pd
from house_buying.calculations import calculate_money_timeseries_after_months
from house_buying.config import load_config
from pathlib import Path

import matplotlib.pyplot as plt
import matplotlib.ticker as mtick


CONFIG_PATH = Path(__file__).parent.joinpath("config", "config.toml")
OUTPUT_PATH = Path(__file__).parent.joinpath("data", "outputs")


def plot_and_save_timeseries(ser: pd.Series, name: str = "output.png"):
    fig, ax = plt.subplots()
    ser.plot(ax=ax, title="Money in Account", style="-")

    ax.set_ylabel("Amount (R$)")
    ax.set_xlabel("Date")
    ax.grid(True, linestyle="--", alpha=0.6)
    ax.set_title("Money in Account", fontsize=14)

    ax.yaxis.set_major_formatter(mtick.StrMethodFormatter("R${x:,.2f}"))

    plt.xticks(rotation=45)

    plt.tight_layout()

    fig.savefig(str(OUTPUT_PATH.joinpath(name)), dpi=300)


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
                ts = calculate_money_timeseries_after_months(
                    config.simulation.months_to_forecast,
                    config.buyer.starting_money,
                    down_payment,
                    house_price,
                    config.house.house_monthly_interest,
                    months,
                    config.buyer.money_saved_monthly,
                    config.buyer.investment_monthly_interest,
                )
                ser = pd.Series(ts)

                plot_and_save_timeseries(
                    ser, f"{int(house_price)}P_{int(down_payment)}D_{months}M.png"
                )


if __name__ == "__main__":
    main()
