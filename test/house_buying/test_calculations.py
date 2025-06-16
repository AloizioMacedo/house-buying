from house_buying.calculations import calculate_left


def test_sanity_check():
    # No interest, and no payment happening.
    assert calculate_left(0, 400_000, 0, 10) == 400_000
    assert calculate_left(0, 328_929, 0, 10) == 328_929

    # No interest, payments happening and finishing.
    assert calculate_left(10_000, 300_000, 0, 30) == 0
    assert calculate_left(20_000, 300_000, 0, 15) == 0

    # No interest, value left.
    assert calculate_left(150, 1000, 0, 5) == 250

    # No interest, overpayment
    assert calculate_left(300, 1000, 0, 5) == -500

    # With interest
    # 1100 -> 800
    # 880 -> 580
    # 638 -> 338
    # 338 + 33.8 = 371.8
    assert abs(calculate_left(300, 1000, 0.1, 3) - 371.8) < 0.001  # Float comparison
