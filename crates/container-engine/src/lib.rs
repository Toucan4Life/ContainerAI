//! Deterministic rules engine for the Container board game (10th Anniversary rules).

pub mod action;
pub mod arrange;
pub mod auction;
pub mod bank;
pub mod build;
pub mod cards;
pub mod color;
pub mod config;
pub mod delivery;
pub mod game;
pub mod game_end;
pub mod loans;
pub mod observation;
pub mod phase;
pub mod produce;
pub mod purchase;
pub mod rng;
pub mod sail;
pub mod scoring;
pub mod seizure;
pub mod state;

#[cfg(test)]
mod tests;
