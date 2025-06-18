from dataclasses import dataclass


@dataclass(slots=True)
class SimulationOutput:
    time_series: list[float]
    monthly_payment: float


def calculate_money_timeseries_after_months(
    months_to_forecast: int,
    starting_money: float,
    down_payment: float,
    house_price: float,
    house_monthly_interest: float,
    n_months_to_pay: int,
    liquid_salary: float,
    fixed_monthly_expenses: float,
    investment_monthly_interest: float,
) -> SimulationOutput:
    """Gets the monthly timeseries of money on account after buying house.

    Args:
        months_to_forecast: How far in the future you want to see data on.
            This can be greater or lower than the number of months
            the payment will take.
        starting_money: Money that is currently on the account before
            buying the house.
        down_payment: Starting money given at the beginning of the payment.
        house_price: Price of the house. Should include the down_payment.
        house_monthly_interest: Monthly interest on which the financing
            is based upon.
        n_months_to_pay: How many months will the payment take.
        liquid_salary: Liquid monthly incoming money to the account
        fixed_monthly_expenses: Monthly expenses other than the monthly
            house payment.
        investment_monthly_interest: Interest of the investments you put
            your money on.

    Returns:
        A timeseries with the information, on a monthly basis, of the money
        that will be on your account.
    """
    ts: list[float] = []

    money_left = starting_money - down_payment
    ts.append(money_left)

    monthly_payment = calculate_monthly_payment(
        house_price - down_payment, house_monthly_interest, n_months_to_pay
    )

    for i in range(months_to_forecast):
        if i <= n_months_to_pay:
            money_left -= monthly_payment

        money_left = money_left * (1 + investment_monthly_interest) + (
            liquid_salary - fixed_monthly_expenses - monthly_payment
        )
        ts.append(money_left)

    return SimulationOutput(
        time_series=ts,
        monthly_payment=monthly_payment,
    )


def calculate_monthly_payment(
    value: float,
    monthly_interest: float,
    n_months: int,
    err: float = 0.001,
    max_iters: int = 10_000,
    upper_bound: int = 100_000,
) -> float:
    """Calculates the monthly payment of a given value with monthly interest.


    Uses binary search to find the value. The last arguments are related to
    the abstract binary search itself, not the specifics of a loan/mortgage/etc.

    Args:
        value: Total value that is being payed.
        monthly_interest: Monthly interest rate.
        n_months: Number of months over which the payment will occur.
        err: Error value for when to decide to finish the binary search.
        max_iters: Maximum iterations for the binary search before giving up.
        upper_bound: Upper bound on the value of the monthly payment.

    Returns:
        Monthly payment that will be required to finish the payment of the
        total value in the allocated number of months.
    """
    a = 0
    b = upper_bound
    c = (a + b) / 2

    for _ in range(max_iters):
        current_error = calculate_left(c, value, monthly_interest, n_months)
        if abs(current_error) < err:
            return c

        if current_error > 0:
            a = c
        else:
            b = c

        c = (a + b) / 2

    return c


def calculate_left(
    monthly_payment: float, total: float, monthly_interest: float, n_months: int
) -> float:
    """Calculates how much money is left to be paid with a given monthly payment.

    If a greater than zero value is left at the end, it finishes
    with the value considering a final computation of interest.
    """
    left = total * (1 + monthly_interest)

    for _ in range(n_months):
        left -= monthly_payment

        if left > 0:
            left *= 1 + monthly_interest

    return left
