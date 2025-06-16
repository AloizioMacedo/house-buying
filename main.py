import pandas as pd
from house_buying.calculations import calculate_money_timeseries_after_months
from house_buying.config import load_config
from pathlib import Path

import matplotlib.pyplot as plt
import matplotlib.ticker as mtick


CONFIG_PATH = Path(__file__).parent.joinpath("data", "config", "config.toml")
OUTPUT_PATH = Path(__file__).parent.joinpath("data", "outputs")


def plot_and_save_timeseries(ser: pd.Series):
    fig, ax = plt.subplots()
    ser.plot(ax=ax, title="Money in Account", style="-")

    ax.set_ylabel("Amount (R$)")
    ax.set_xlabel("Date")
    ax.grid(True, linestyle="--", alpha=0.6)
    ax.set_title("Money in Account", fontsize=14)

    ax.yaxis.set_major_formatter(mtick.StrMethodFormatter("R${x:,.2f}"))

    plt.xticks(rotation=45)

    plt.tight_layout()

    fig.savefig(str(OUTPUT_PATH.joinpath("output.png")), dpi=300)


def main():
    config = load_config(str(CONFIG_PATH))

    ts = calculate_money_timeseries_after_months(
        config.simulation.months_to_forecast,
        config.buyer.starting_money,
        config.house.down_payment,
        config.house.house_price,
        config.house.house_monthly_interest,
        config.house.months_to_pay,
        config.buyer.money_saved_monthly,
        config.buyer.investment_monthly_interest,
    )
    ser = pd.Series(ts)
    plot_and_save_timeseries(ser)


if __name__ == "__main__":
    main()
