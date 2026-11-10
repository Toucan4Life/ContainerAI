//! Browser bindings for the Container engine. All the game logic lives in `container-engine`; the UI
//! only sends the index of a legal action and draws the JSON snapshot it gets back.

use wasm_bindgen::prelude::*;

mod dto;
mod session;

pub use session::Session;

/// The opponents the UI can seat, as a JSON list of `{id, name, description}`.
#[wasm_bindgen]
pub fn opponents() -> String {
    opponents_json()
}

pub fn opponents_json() -> String {
    let list: Vec<_> = container_bots::OPPONENTS
        .iter()
        .map(|o| serde_json::json!({ "id": o.id, "name": o.name, "description": o.description }))
        .collect();
    serde_json::Value::Array(list).to_string()
}

/// A game for the browser. Every method returns JSON text.
#[wasm_bindgen]
pub struct Game {
    session: Session,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(config: &str) -> Result<Game, JsError> {
        Ok(Game {
            session: Session::new(config).map_err(|e| JsError::new(&e))?,
        })
    }

    pub fn snapshot(&self) -> String {
        self.session.snapshot().to_string()
    }

    /// Plays the `id`th legal action offered by the last snapshot.
    pub fn act(&mut self, id: usize) -> Result<(), JsError> {
        self.session.act(id).map_err(|e| JsError::new(&e))
    }

    /// Takes back the last single step, such as one container placement.
    pub fn step_back(&mut self) -> Result<(), JsError> {
        self.session.step_back().map_err(|e| JsError::new(&e))
    }

    /// Takes back the last action of the human's turn (or cancels the one in progress).
    pub fn undo(&mut self) -> Result<(), JsError> {
        self.session.undo().map_err(|e| JsError::new(&e))
    }

    /// Lets the computer opponent whose turn it is take one action. False if it is not a bot's turn.
    pub fn step(&mut self) -> Result<bool, JsError> {
        self.session.step().map_err(|e| JsError::new(&e))
    }
}

#[cfg(test)]
mod tests;
