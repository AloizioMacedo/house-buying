import tomllib
from pydantic import BaseModel


class Simulation(BaseModel):
    months_to_forecast: int


class Buyer(BaseModel):
    starting_money: float
    investment_monthly_interest: float
    liquid_salary: float
    fixed_monthly_expenses: float


class House(BaseModel):
    house_price: float | list[float]
    down_payment: float | list[float]
    months_to_pay: int | list[int]
    house_monthly_interest: float


class Config(BaseModel):
    simulation: Simulation
    buyer: Buyer
    house: House


def load_config(path: str) -> Config:
    with open(path) as f:
        contents = f.read()
        loaded_config = tomllib.loads(contents)

        config = Config.model_validate(loaded_config)

    return config
