from house_buying.config import load_config
from pathlib import Path

HERE = Path(__file__).parent


def test_config_parsing():
    config = load_config(str(HERE.joinpath("data", "config.toml")))

    assert config.buyer.starting_money == 1_000_000
    assert config.buyer.money_saved_monthly == 10_000

    assert config.house.house_price == 500_000
    assert config.house.down_payment == 150_000

    assert config.simulation.months_to_forecast == 48
