import pandas as pd
from house_buying.calculations import calculate_money_timeseries_after_months
from house_buying.config import load_config
from pathlib import Path

import matplotlib.pyplot as plt

CONFIG_PATH = Path(__file__).parent.joinpath("data", "config", "config.toml")
OUTPUT_PATH = Path(__file__).parent.joinpath("data", "outputs")


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
    fig = ser.plot(title="Money in account").get_figure()
    fig.savefig(str(OUTPUT_PATH.joinpath("output.png")))


if __name__ == "__main__":
    main()
