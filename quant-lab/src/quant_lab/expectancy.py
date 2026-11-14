"""Fee-aware expectancy helpers for already-netted trade outcomes."""

from __future__ import annotations


def net_expectancy(
    win_probability: float,
    average_net_win: float,
    average_net_loss: float,
) -> float:
    """Return E = P(win)*avg_net_win - P(loss)*avg_net_loss.

    Payoffs must already include trading fees, funding and slippage. The loss
    argument is the positive magnitude of the average net losing trade.
    """
    if not 0.0 <= win_probability <= 1.0:
        raise ValueError("win_probability must be between 0 and 1")
    if average_net_win < 0.0 or average_net_loss < 0.0:
        raise ValueError("average net win/loss magnitudes must be non-negative")
    return (
        win_probability * average_net_win
        - (1.0 - win_probability) * average_net_loss
    )
