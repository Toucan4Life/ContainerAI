"""ctypes wrapper around the batched game environment (`crates/container-env`)."""

from __future__ import annotations

import ctypes as C
import os
from pathlib import Path

import numpy as np

RANDOM, SIMPLE, MERCHANT, TRADER = -1, -2, -3, -4
MAX_PLAYERS = 5
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
        lib.cenv_breakdown.argtypes = [C.c_void_p, C.POINTER(C.c_int)]
        lib.cenv_traded.argtypes = [C.c_void_p, C.POINTER(C.c_int)]
        lib.cenv_decision_info.argtypes = [C.c_void_p, C.POINTER(C.c_int), C.POINTER(C.c_int)]
        lib.cenv_observe.argtypes = [
            C.c_void_p, C.POINTER(C.c_float), C.POINTER(C.c_float), C.POINTER(C.c_uint8),
            C.POINTER(C.c_int), C.POINTER(C.c_int),
        ]
        lib.cenv_ban_bank.argtypes = [C.c_void_p, C.c_int]
        lib.cenv_ban_bank_all.argtypes = [C.c_void_p, C.c_int]
        lib.cenv_require_harbor_purchase.argtypes = [C.c_void_p, C.c_int]
        lib.cenv_forbid_same_turn_repay.argtypes = [C.c_void_p, C.c_int]
        lib.cenv_step.argtypes = [C.c_void_p, C.POINTER(C.c_int)]
        lib.cenv_teacher.argtypes = [C.c_void_p, C.POINTER(C.c_int)]
        lib.cenv_results.argtypes = [C.c_void_p] + [C.POINTER(C.c_int)] * 3
        self.lib = lib
        self.n = n
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

    def require_min_bid(self, slot: int, amount: int) -> None:
        """Seats of this slot open a delivery auction with at least `amount` (their whole cash if less)."""
        self.lib.cenv_set_min_bid(self.handle, slot, amount)

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
