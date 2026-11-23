"""ctypes wrapper around the batched game environment (`crates/container-env`)."""

from __future__ import annotations

import ctypes as C
import os
from pathlib import Path

import numpy as np

RANDOM, SIMPLE, MERCHANT, TRADER, DISCOUNTER, BARGAIN, SOLITAIRE = -1, -2, -3, -4, -5, -6, -7
PRODUCER, SHOWCASE, SHIPPER = -8, -9, -10
MAX_PLAYERS = 5
INTERACTION = 92  # columns of `BatchEnv.interaction`, see `crates/container-env`
(BOUGHT, SPENT, SOLD, EARNED, BIDS, BID_MONEY, REAL_BIDS, PLACED_AT) = range(8)  # PLACED_AT..PLACED_AT+5: laid out in a $1..$6 lot
# Strategy diagnostics (`training/strategy_metrics.py` has the formulas); the colour blocks have five columns each.
(MAIN_DECISIONS, PRODUCE_CHANCES, PRODUCES, PRODUCED, OFFERED, OFFERED_COMPETITIVE, COMPETITIVE_COLORS) = range(13, 20)
BOUGHT_COLOR, SOLD_COLOR = 20, 25
DELIVERIES, SHIPPED, SUBSIDY = 30, 31, 32
# Per delivery bid, summed: the bidder's allowance for the cargo (its value on the bidder's scoring card, $5 at least), and the money bid above that.
BID_VALUE, OVERBID = 33, 34
# Trips to the Island by cargo of 1..5 containers (CARGO_SIZE..+5), the deliverer's cash for accepted deliveries of each size (CARGO_CASH..+5,
# twice the winning bid), distinct colours of all cargoes; summed over Main decisions: factories, warehouses owned, harbor containers laid out;
# builds made by the build script.
CARGO_SIZE, CARGO_CASH, CARGO_COLORS = 35, 40, 45
FACTORIES_HELD, WAREHOUSES_HELD, HARBOR_OFFERED, SCRIPT_BUILDS = 46, 47, 48, 49
# Per delivery bid: the bidder's cash before it bids; in the deliverer's row, the rivals' bids and how many could afford $3 and $10.
BID_CASH, RIVAL_BIDS, RIVAL_CASH_3, RIVAL_CASH_10 = 50, 51, 52, 53
# Cash flows for the ledger (`training/ledger.py`): delivery auctions the seat won as bidder and what it paid, what it paid the Bank to buy
# its own cargo out, and the interest due when its turns begin ($1 per loan held; unpaid only in a default, which seizes containers).
DEALS_WON, DEAL_PAID, BUYOUT_PAID, INTEREST = 54, 55, 56, 57
MOVES = 58  # from here, one counter per kind of move (`move_names`): every action the seat plays, in any phase
IDLE, PENDING, DONE, STALLED = 0, 1, 2, 3
SHORT, STANDARD, EXTENDED = 0, 1, 2
LENGTH_NAMES = {SHORT: "short", STANDARD: "standard", EXTENDED: "extended"}
PHASE_NAMES = ["main", "produce", "buy", "load", "arrange", "bid", "decide", "seize", "distribute",
               "bank_bid_containers", "bank_bid_cash", "game_over"]


def find_library() -> Path:
    if os.environ.get("CONTAINER_ENV_LIB"):
        return Path(os.environ["CONTAINER_ENV_LIB"])
    root = Path(__file__).resolve().parent.parent
    targets = [os.environ.get("CARGO_TARGET_DIR"), root / "target", "/home/vscode/.cargo-target"]
    for t in filter(None, targets):
        p = Path(t) / "release" / "libcontainer_env.so"
        if p.exists():
            return p
    raise FileNotFoundError("build it first: cargo build --release -p container-env")


def _ptr(a: np.ndarray, ctype):
    return a.ctypes.data_as(C.POINTER(ctype))


def _check_width(lib) -> None:
    """The library fills `INTERACTION` columns per seat; another width means it is stale and would misalign the rows."""
    try:
        lib.cenv_interaction_width.restype = C.c_size_t
        width = lib.cenv_interaction_width()
    except AttributeError:
        width = None
    if width != INTERACTION:
        raise RuntimeError(f"libcontainer_env.so has {width} interaction columns, not {INTERACTION}: "
                           "rebuild it with `cargo build --release -p container-env`")


def move_names() -> list[str]:
    """The kinds of move counted by the columns of `BatchEnv.interaction` from `MOVES` on, in column order."""
    lib = C.CDLL(str(find_library()))
    _check_width(lib)
    lib.cenv_move_names.restype = C.c_char_p
    return lib.cenv_move_names().decode().rstrip(",").split(",")


def _buffer(shape, dtype, pinned: bool) -> np.ndarray:
    """A zeroed array; pinned host memory makes the copies to the GPU faster."""
    if pinned:
        import torch

        return torch.zeros(shape, dtype=torch.from_numpy(np.zeros(0, dtype)).dtype, pin_memory=True).numpy()
    return np.zeros(shape, dtype=dtype)


class BatchEnv:
    """`n` games played in lockstep. See the Rust crate for the call order."""

    def __init__(self, n: int, threads: int = 8, pinned: bool = False):
        lib = C.CDLL(str(find_library()))
        _check_width(lib)
        lib.cenv_new.restype = C.c_void_p
        lib.cenv_new.argtypes = [C.c_size_t, C.c_size_t]
        for name in ("cenv_free", "cenv_advance"):
            getattr(lib, name).argtypes = [C.c_void_p]
        lib.cenv_dims.argtypes = [C.POINTER(C.c_uint32)]
        lib.cenv_reset.argtypes = [C.c_void_p, C.c_size_t, C.c_size_t, C.c_uint64, C.POINTER(C.c_int)]
        lib.cenv_reset_ex.argtypes = [C.c_void_p, C.c_size_t, C.c_size_t, C.c_uint64, C.c_int, C.POINTER(C.c_int)]
        lib.cenv_set_horizon.argtypes = [C.c_void_p, C.c_size_t]
        lib.cenv_results_ex.argtypes = [C.c_void_p] + [C.POINTER(C.c_int)] * 5
        lib.cenv_set_min_bid.argtypes = [C.c_void_p, C.c_int, C.c_uint32]
        lib.cenv_set_max_price.argtypes = [C.c_void_p, C.c_int, C.c_uint32]
        lib.cenv_breakdown.argtypes = [C.c_void_p, C.POINTER(C.c_int)]
        lib.cenv_traded.argtypes = [C.c_void_p, C.POINTER(C.c_int)]
        lib.cenv_interaction.argtypes = [C.c_void_p, C.POINTER(C.c_int)]
        lib.cenv_decision_info.argtypes = [C.c_void_p, C.POINTER(C.c_int), C.POINTER(C.c_int)]
        lib.cenv_observe.argtypes = [
            C.c_void_p, C.POINTER(C.c_float), C.POINTER(C.c_float), C.POINTER(C.c_uint8),
            C.POINTER(C.c_int), C.POINTER(C.c_int),
        ]
        lib.cenv_ban_bank.argtypes = [C.c_void_p, C.c_int]
        lib.cenv_ban_bank_all.argtypes = [C.c_void_p, C.c_int]
        lib.cenv_require_harbor_purchase.argtypes = [C.c_void_p, C.c_int]
        lib.cenv_forbid_double_reprice.argtypes = [C.c_void_p, C.c_int]
        lib.cenv_require_bank_load.argtypes = [C.c_void_p, C.c_int]
        lib.cenv_require_bid_within_value.argtypes = [C.c_void_p, C.c_int]
        lib.cenv_set_build_script.argtypes = [C.c_void_p, C.c_int, C.c_int, C.c_int, C.c_int, C.c_uint32]
        lib.cenv_set_value_bidder.argtypes = [C.c_void_p, C.c_int, C.c_uint32, C.c_uint32]
        lib.cenv_set_min_cargo.argtypes = [C.c_void_p, C.c_int, C.c_uint32]
        lib.cenv_mirror_rules.argtypes = [C.c_void_p, C.c_int, C.c_int]
        lib.cenv_mirror_rules_set.argtypes = [C.c_void_p, C.c_uint32, C.c_int]
        lib.cenv_forbid_same_turn_repay.argtypes = [C.c_void_p, C.c_int]
        lib.cenv_step.argtypes = [C.c_void_p, C.POINTER(C.c_int)]
        lib.cenv_teacher.argtypes = [C.c_void_p, C.POINTER(C.c_int)]
        lib.cenv_results.argtypes = [C.c_void_p] + [C.POINTER(C.c_int)] * 3
        self.lib = lib
        self.n = n
        lib.cenv_set_memory.argtypes = [C.c_int]
        lib.cenv_set_memory(int(os.environ.get("CAI_MEMORY", "0")))
        dims = np.zeros(3, dtype=np.uint32)
        lib.cenv_dims(_ptr(dims, C.c_uint32))
        self.state_dim, self.action_dim, self.max_actions = (int(x) for x in dims)
        self.handle = lib.cenv_new(n, threads)
        self.state = _buffer((n, self.state_dim), np.float32, pinned)
        self.acts = _buffer((n, self.max_actions, self.action_dim), np.float32, pinned)
        self.mask = _buffer((n, self.max_actions), np.uint8, pinned)
        self.slot = np.zeros(n, dtype=np.int32)
        self.seat = np.zeros(n, dtype=np.int32)
        self._choice = np.zeros(n, dtype=np.int32)
        self._teacher = np.zeros(n, dtype=np.int32)
        self._status = np.zeros(n, dtype=np.int32)
        self._players = np.zeros(n, dtype=np.int32)
        self._scores = np.zeros((n, MAX_PLAYERS), dtype=np.int32)
        self._traded = np.zeros((n, MAX_PLAYERS), dtype=np.int32)
        self._breakdown = np.zeros((n, MAX_PLAYERS, 4), dtype=np.int32)
        self._interaction = np.zeros((n, MAX_PLAYERS, INTERACTION), dtype=np.int32)
        self._winners = np.zeros(n, dtype=np.int32)
        self._actions = np.zeros(n, dtype=np.int32)
        self._legal = np.zeros(n, dtype=np.int32)
        self._phase = np.zeros(n, dtype=np.int32)

    def close(self) -> None:
        if self.handle:
            self.lib.cenv_free(self.handle)
            self.handle = None

    def __del__(self):
        try:
            self.close()
        except Exception:
            pass

    def ban_bank(self, slot: int) -> None:
        """Seats of this slot can no longer call the Bank (start or outbid an auction)."""
        self.lib.cenv_ban_bank(self.handle, slot)

    def ban_bank_all(self, on: bool = True) -> None:
        """Nobody at the table can call the Bank, scripted seats included (the classic rule)."""
        self.lib.cenv_ban_bank_all(self.handle, int(on))

    def require_harbor_purchase(self, slot: int) -> None:
        """Seats of this slot must buy something whenever they sail to a harbor."""
        self.lib.cenv_require_harbor_purchase(self.handle, slot)

    def forbid_double_reprice(self, slot: int) -> None:
        """Seats of this slot may not reprice the same district twice in a row within a turn."""
        self.lib.cenv_forbid_double_reprice(self.handle, slot)

    def require_bank_load(self, slot: int) -> None:
        """Seats of this slot may only sail to the Bank when they have a container there to load."""
        self.lib.cenv_require_bank_load(self.handle, slot)

    def mirror_rules(self, slot: int, like: int) -> None:
        """Seats of `slot` play under the rules that bind `like` (a frozen copy of a net trained under them)."""
        self.lib.cenv_mirror_rules(self.handle, slot, like)

    def mirror_rules_set(self, mask: int, like: int) -> None:
        """Net slots `SLOT_BASE + k` with bit `k` in `mask` play under the rules that bind `like`."""
        self.lib.cenv_mirror_rules_set(self.handle, mask, like)

    def require_min_bid(self, slot: int, amount: int) -> None:
        """Seats of this slot open a delivery auction with at least `amount` (their whole cash if less)."""
        self.lib.cenv_set_min_bid(self.handle, slot, amount)

    def require_max_price(self, slot: int, amount: int) -> None:
        """Seats of this slot lay no container out in a lot that costs more than `amount`."""
        self.lib.cenv_set_max_price(self.handle, slot, amount)

    def require_bid_within_value(self, slot: int) -> None:
        """Seats of this slot never have more on the table in a delivery auction than the cargo is worth on their scoring card ($5 at least)."""
        self.lib.cenv_require_bid_within_value(self.handle, slot)

    def build_script(self, slot: int, warehouse: bool = False, count: int = 1, after_main: int = 0, reserve: int = 5) -> None:
        """Investment probe: seats of this slot build `count` factories (or warehouses) at their first Main decision after `after_main`
        of their own where the build is legal and leaves `reserve` cash; the net plays everything else. A negative slot lifts it."""
        self.lib.cenv_set_build_script(self.handle, slot, int(warehouse), count, after_main, reserve)

    def value_bidders(self, except_slot: int, percent: int, lend: bool = False) -> None:
        """Probe: every seat except those of `except_slot` bids for deliveries by `percent` of the cargo's value on its own card (0 lifts it).

        With `lend`, a bidder that lacks the cash for its bid is lent the difference until the auction is settled.
        """
        self.lib.cenv_set_value_bidder(self.handle, except_slot, percent, int(lend))

    def require_min_cargo(self, slot: int, containers: int) -> None:
        """Probe: seats of this slot do not sail to the Island with fewer containers on the ship (waived if nothing else is legal)."""
        self.lib.cenv_set_min_cargo(self.handle, slot, containers)

    def forbid_same_turn_repay(self, slot: int) -> None:
        """Seats of this slot may not repay a loan in a turn in which they requested one."""
        self.lib.cenv_forbid_same_turn_repay(self.handle, slot)

    def reset(self, i: int, players: int, seed: int, kinds: list[int], length: int = STANDARD) -> None:
        k = np.asarray(kinds[:players], dtype=np.int32)
        if length == STANDARD:
            self.lib.cenv_reset(self.handle, i, players, seed, _ptr(k, C.c_int))
        else:
            self.lib.cenv_reset_ex(self.handle, i, players, seed, length, _ptr(k, C.c_int))

    def set_horizon(self, horizon: int) -> None:
        """Games that have taken this many actions without ending are stalled (from the next advance)."""
        self.lib.cenv_set_horizon(self.handle, horizon)

    def advance(self) -> None:
        self.lib.cenv_advance(self.handle)

    def observe(self):
        """Fills and returns (state, acts, mask, slot, seat); slot is -1 where nothing is pending."""
        self.lib.cenv_observe(
            self.handle, _ptr(self.state, C.c_float), _ptr(self.acts, C.c_float),
            _ptr(self.mask, C.c_uint8), _ptr(self.slot, C.c_int), _ptr(self.seat, C.c_int),
        )
        return self.state, self.acts, self.mask, self.slot, self.seat

    def step(self, choice: np.ndarray) -> None:
        self._choice[:] = choice
        self.lib.cenv_step(self.handle, _ptr(self._choice, C.c_int))

    def teacher(self) -> np.ndarray:
        self.lib.cenv_teacher(self.handle, _ptr(self._teacher, C.c_int))
        return self._teacher

    def results(self):
        """Returns (status, players, scores) for every env."""
        self.lib.cenv_results(
            self.handle, _ptr(self._status, C.c_int), _ptr(self._players, C.c_int),
            _ptr(self._scores, C.c_int),
        )
        return self._status, self._players, self._scores

    def results_ex(self):
        """Returns (status, players, scores, winners, actions): the winners as a bit mask per env (bit p
        for seat p, engine tie-break applied, 0 unless the game is done) and the engine actions taken."""
        self.lib.cenv_results_ex(
            self.handle, _ptr(self._status, C.c_int), _ptr(self._players, C.c_int),
            _ptr(self._scores, C.c_int), _ptr(self._winners, C.c_int), _ptr(self._actions, C.c_int),
        )
        return self._status, self._players, self._scores, self._winners, self._actions

    def breakdown(self) -> np.ndarray:
        """Per env, seat, of finished games: cash, island, leftovers and loans (positive) of the final score."""
        self.lib.cenv_breakdown(self.handle, _ptr(self._breakdown, C.c_int))
        return self._breakdown

    def traded(self) -> np.ndarray:
        """Per env and seat, the money of the player-to-player deliveries the seat took part in (accepted
        bids, as deliverer or winning bidder) in the game so far."""
        self.lib.cenv_traded(self.handle, _ptr(self._traded, C.c_int))
        return self._traded

    def interaction(self) -> np.ndarray:
        """Per env and seat, counters of how the seat dealt with the others so far: containers bought from
        opponents and their cost, containers sold to them and what they brought, delivery bids (and their
        money, and those of at least $2), and the containers laid out in a lot of each price $1..$6."""
        self.lib.cenv_interaction(self.handle, _ptr(self._interaction, C.c_int))
        return self._interaction

    def decision_info(self):
        """Returns (legal, phase) per env: legal actions before thinning and the phase code, -1 if idle."""
        self.lib.cenv_decision_info(self.handle, _ptr(self._legal, C.c_int), _ptr(self._phase, C.c_int))
        return self._legal, self._phase


def win_fraction(scores: np.ndarray, players: int) -> np.ndarray:
    """Per seat, the share of opponents it beat (ties count half), in 0..1."""
    s = scores[:players]
    out = np.zeros(players)
    for i in range(players):
        beaten = sum(1.0 if s[i] > s[j] else 0.5 if s[i] == s[j] else 0.0
                     for j in range(players) if j != i)
        out[i] = beaten / (players - 1)
    return out
