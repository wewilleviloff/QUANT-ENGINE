import pytest

from quant_lab.expectancy import net_expectancy


def test_net_expectancy() -> None:
    assert net_expectancy(0.55, 2.0, 1.0) == pytest.approx(0.65)


@pytest.mark.parametrize("probability", [-0.01, 1.01])
def test_probability_must_be_valid(probability: float) -> None:
    with pytest.raises(ValueError):
        net_expectancy(probability, 2.0, 1.0)


def test_payoff_magnitudes_must_be_non_negative() -> None:
    with pytest.raises(ValueError):
        net_expectancy(0.5, -1.0, 1.0)
