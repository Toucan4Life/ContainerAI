//! A batch of games that Python plays through, one decision at a time.
//!
//! Each seat of each game is either a scripted bot (played here) or a *slot*: a decision that Python
//! answers, usually with a neural net. The calls go: `reset` the games, `advance` them to the next
//! decision of a slot, `observe` what the slots see, then `step` with the choices and repeat.
//!
//! Seat kinds in `reset`: `-1` random, `-2` simple, `-3` merchant, `>= 0` a slot, whose number tells
//! Python which policy answers.

use std::os::raw::c_int;

use container_bots::features::{
    ACTION_DIM, MAX_ACTIONS, STATE_DIM, encode_action, encode_state, thin,
};
use container_bots::{Opponent, create};
use container_engine::action::Action;
use container_engine::config::{GameConfig, MAX_PLAYERS};
use container_engine::game::Game;

/// A game that has run this long is stalled and ends without a result. Real games take 650-800
/// actions; a longer limit only lets a stalling policy pile up stored decisions.
const STALL: usize = 2_500;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Idle,
    Pending,
    Done,
    Stalled,
}

struct Env {
    game: Option<Game>,
    kinds: [i32; MAX_PLAYERS],
    scripted: Vec<Option<Box<dyn Opponent + Send>>>,
    teacher: Option<Box<dyn Opponent + Send>>,
    legal: Vec<Action>,
    thinned: Vec<usize>,
    actions: usize,
    players: usize,
    status: Status,
}

impl Env {
    fn new() -> Self {
        Self {
            game: None,
            kinds: [-1; MAX_PLAYERS],
            scripted: Vec::new(),
            teacher: None,
            legal: Vec::new(),
            thinned: Vec::new(),
            actions: 0,
            players: 0,
            status: Status::Idle,
        }
    }

    fn reset(&mut self, players: usize, seed: u64, kinds: &[i32]) {
        self.game = Game::new(GameConfig::new(players).with_seed(seed)).ok();
        self.kinds = [-1; MAX_PLAYERS];
        self.kinds[..players].copy_from_slice(&kinds[..players]);
        self.scripted = (0..players)
            .map(|seat| match kinds[seat] {
                -1 => create("random", seed ^ (seat as u64 + 1)),
                -2 => create("simple", seed ^ (seat as u64 + 1)),
                -3 => create("merchant", seed ^ (seat as u64 + 1)),
                _ => None,
            })
            .collect();
        self.teacher = create("merchant", seed ^ 0x7EAC4E5);
        self.actions = 0;
        self.players = players;
        self.status = Status::Idle;
    }

    /// Plays scripted seats and forced moves until a slot has a real choice, or the game ends.
    fn advance(&mut self) {
        if self.status != Status::Idle {
            return;
        }
        let Some(game) = self.game.as_mut() else { return };
        loop {
            if game.is_terminal() {
                self.status = Status::Done;
                return;
            }
            if self.actions >= STALL {
                self.status = Status::Stalled;
                return;
            }
            let seat = game.current_player();
            let legal = game.legal_actions();
            let action = if let Some(bot) = self.scripted[seat].as_mut() {
                let obs = game.observation(seat).expect("seat");
                bot.choose(&obs, &legal)
            } else if legal.len() == 1 {
                legal[0]
            } else {
                self.thinned = thin(legal.len());
                self.legal = legal;
                self.status = Status::Pending;
                return;
            };
            game.apply(action).expect("legal action");
            self.actions += 1;
        }
    }

    fn observe(&self, state: &mut [f32], acts: &mut [f32], mask: &mut [u8]) -> Option<usize> {
        let game = self.game.as_ref()?;
        if self.status != Status::Pending {
            return None;
        }
        let seat = game.current_player();
        let obs = game.observation(seat)?;
        encode_state(&obs, state);
        for (k, &i) in self.thinned.iter().enumerate() {
            encode_action(&obs, self.legal[i], &mut acts[k * ACTION_DIM..(k + 1) * ACTION_DIM]);
            mask[k] = 1;
        }
        Some(seat)
    }

    fn step(&mut self, choice: usize) {
        if self.status != Status::Pending {
            return;
        }
        let action = self.legal[self.thinned[choice.min(self.thinned.len() - 1)]];
        self.game.as_mut().expect("game").apply(action).expect("legal action");
        self.actions += 1;
        self.status = Status::Idle;
    }

    /// What the merchant would do here, as an index into the thinned list.
    fn teach(&mut self) -> Option<usize> {
        if self.status != Status::Pending {
            return None;
        }
        let game = self.game.as_ref()?;
        let obs = game.observation(game.current_player())?;
        let pick = self.teacher.as_mut()?.choose(&obs, &self.legal);
        let at = self.legal.iter().position(|&a| a == pick)?;
        // The thinned action nearest to the teacher's choice.
        (0..self.thinned.len()).min_by_key(|&k| self.thinned[k].abs_diff(at))
    }
}

pub struct Batch {
    envs: Vec<Env>,
    threads: usize,
}

impl Batch {
    fn each<F: Fn(usize, &mut Env) + Sync>(&mut self, f: F) {
        let chunk = self.envs.len().div_ceil(self.threads.max(1)).max(1);
        std::thread::scope(|scope| {
            for (c, envs) in self.envs.chunks_mut(chunk).enumerate() {
                let f = &f;
                scope.spawn(move || {
                    for (i, env) in envs.iter_mut().enumerate() {
                        f(c * chunk + i, env);
                    }
                });
            }
        });
    }
}

/// A raw pointer that may cross threads; every thread touches only its own env's part of the buffer.
#[derive(Clone, Copy)]
struct Shared<T>(*mut T);
unsafe impl<T> Send for Shared<T> {}
unsafe impl<T> Sync for Shared<T> {}

#[unsafe(no_mangle)]
pub extern "C" fn cenv_new(n: usize, threads: usize) -> *mut Batch {
    let envs = (0..n).map(|_| Env::new()).collect();
    Box::into_raw(Box::new(Batch { envs, threads }))
}

/// # Safety
/// `b` comes from `cenv_new` and is not used afterwards.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_free(b: *mut Batch) {
    drop(unsafe { Box::from_raw(b) });
}

/// Writes `[STATE_DIM, ACTION_DIM, MAX_ACTIONS]` into `out`.
///
/// # Safety
/// `out` points to three `u32`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_dims(out: *mut u32) {
    let out = unsafe { std::slice::from_raw_parts_mut(out, 3) };
    out.copy_from_slice(&[STATE_DIM as u32, ACTION_DIM as u32, MAX_ACTIONS as u32]);
}

/// Starts a new game in env `i`. `kinds` holds one seat kind per player.
///
/// # Safety
/// `b` is live, `i` is in range and `kinds` points to `players` values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_reset(
    b: *mut Batch,
    i: usize,
    players: usize,
    seed: u64,
    kinds: *const c_int,
) {
    let batch = unsafe { &mut *b };
    let kinds = unsafe { std::slice::from_raw_parts(kinds, players) };
    batch.envs[i].reset(players, seed, kinds);
}

/// Plays every env up to its next decision.
///
/// # Safety
/// `b` is live.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_advance(b: *mut Batch) {
    unsafe { &mut *b }.each(|_, env| env.advance());
}

/// Writes each env's pending decision; rows of envs with nothing pending are left as they were.
/// Buffers are `[n, STATE_DIM]`, `[n, MAX_ACTIONS, ACTION_DIM]`
/// and `[n, MAX_ACTIONS]`, and padding is zeroed; `slot` and `seat` get -1 where nothing is pending.
///
/// # Safety
/// `b` is live and every buffer has the size above.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_observe(
    b: *mut Batch,
    state: *mut f32,
    acts: *mut f32,
    mask: *mut u8,
    slot: *mut c_int,
    seat: *mut c_int,
) {
    let batch = unsafe { &mut *b };
    let (state, acts, mask) = (Shared(state), Shared(acts), Shared(mask));
    let (slot, seat) = (Shared(slot), Shared(seat));
    batch.each(move |i, env| {
        let (state, acts, mask, slot, seat) = (state, acts, mask, slot, seat);
        let (s, a, m) = unsafe {
            (
                std::slice::from_raw_parts_mut(state.0.add(i * STATE_DIM), STATE_DIM),
                std::slice::from_raw_parts_mut(
                    acts.0.add(i * MAX_ACTIONS * ACTION_DIM),
                    MAX_ACTIONS * ACTION_DIM,
                ),
                std::slice::from_raw_parts_mut(mask.0.add(i * MAX_ACTIONS), MAX_ACTIONS),
            )
        };
        m.fill(0);
        if env.status == Status::Pending {
            s.fill(0.0);
            a.fill(0.0);
        }
        let who = env.observe(s, a, m);
        unsafe {
            *seat.0.add(i) = who.map_or(-1, |w| w as c_int);
            *slot.0.add(i) = who.map_or(-1, |w| env.kinds[w]);
        }
    });
}

/// Applies one choice per env (an index into the thinned action list; ignored where nothing is pending).
///
/// # Safety
/// `b` is live and `choice` points to one value per env.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_step(b: *mut Batch, choice: *const c_int) {
    let batch = unsafe { &mut *b };
    let choice = Shared(choice as *mut c_int);
    batch.each(move |i, env| {
        let choice = choice;
        env.step(unsafe { *choice.0.add(i) }.max(0) as usize)
    });
}

/// Writes what the merchant would choose in each pending decision, or -1.
///
/// # Safety
/// `b` is live and `out` points to one value per env.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_teacher(b: *mut Batch, out: *mut c_int) {
    let batch = unsafe { &mut *b };
    let out = Shared(out);
    batch.each(move |i, env| {
        let out = out;
        unsafe { *out.0.add(i) = env.teach().map_or(-1, |k| k as c_int) };
    });
}

/// Writes each env's status (0 idle, 1 pending, 2 done, 3 stalled) and final scores `[n, MAX_PLAYERS]`
/// (zero until the game is done), plus the number of players.
///
/// # Safety
/// `b` is live; `status` and `players` hold one value per env and `scores` `MAX_PLAYERS` per env.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cenv_results(
    b: *mut Batch,
    status: *mut c_int,
    players: *mut c_int,
    scores: *mut c_int,
) {
    let batch = unsafe { &mut *b };
    for (i, env) in batch.envs.iter().enumerate() {
        let s = match env.status {
            Status::Idle => 0,
            Status::Pending => 1,
            Status::Done => 2,
            Status::Stalled => 3,
        };
        unsafe {
            *status.add(i) = s;
            *players.add(i) = env.players as c_int;
            for p in 0..MAX_PLAYERS {
                *scores.add(i * MAX_PLAYERS + p) = 0;
            }
            if env.status == Status::Done {
                if let Some(r) = env.game.as_ref().and_then(|g| g.results()) {
                    for (p, sc) in r.scores.iter().enumerate() {
                        *scores.add(i * MAX_PLAYERS + p) = sc.total;
                    }
                }
            }
        }
    }
}
