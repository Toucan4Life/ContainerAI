//! The trained nets the opponents run on, by the name of their weights file (`psro08` is
//! `weights/psro08.bin`).
//!
//! With the `embedded-nets` feature, which is on by default, every net is built into the crate and parsed
//! on first use. Without it a net exists only once [`provide`] has been given its bytes. The browser build
//! works that way: the page fetches the one net a game seats instead of carrying all nine.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::neural::Net;
use crate::{adaptive, broker, classic, consortium, dealer, haggler, magnate, peddler, peddler2};

#[cfg(feature = "embedded-nets")]
const EMBEDDED: [(&str, &[u8]); 9] = [
    (adaptive::NET, include_bytes!("../weights/adaptive.bin")),
    (broker::NET, include_bytes!("../weights/broker.bin")),
    (classic::NET, include_bytes!("../weights/classic.bin")),
    (consortium::NET, include_bytes!("../weights/psro08.bin")),
    (dealer::NET, include_bytes!("../weights/dealer.bin")),
    (haggler::NET, include_bytes!("../weights/haggler.bin")),
    (magnate::NET, include_bytes!("../weights/magnate.bin")),
    (peddler::NET, include_bytes!("../weights/peddler.bin")),
    (peddler2::NET, include_bytes!("../weights/peddler2.bin")),
];

fn registry() -> &'static Mutex<HashMap<String, Arc<Net>>> {
    static NETS: OnceLock<Mutex<HashMap<String, Arc<Net>>>> = OnceLock::new();
    NETS.get_or_init(Default::default)
}

fn embedded(name: &str) -> Option<&'static [u8]> {
    #[cfg(feature = "embedded-nets")]
    {
        EMBEDDED.iter().find(|(n, _)| *n == name).map(|(_, bytes)| *bytes)
    }
    #[cfg(not(feature = "embedded-nets"))]
    {
        let _ = name;
        None
    }
}

/// Whether [`get`] can return the net called `name`.
pub fn available(name: &str) -> bool {
    embedded(name).is_some() || registry().lock().is_ok_and(|nets| nets.contains_key(name))
}

/// Parses `bytes`, the contents of a weights file, and keeps the net under `name` for every opponent
/// that plays it. A net provided twice is replaced.
pub fn provide(name: &str, bytes: &[u8]) -> Result<(), String> {
    let net = Net::from_bytes(bytes).map_err(|e| format!("the weights of net {name} are invalid: {e:?}"))?;
    registry()
        .lock()
        .map_err(|_| "the net registry is poisoned".to_string())?
        .insert(name.to_string(), Arc::new(net));
    Ok(())
}

/// The net called `name`, shared by every seat that plays it.
///
/// # Panics
/// If the net is not [`available`]. [`crate::create`] checks that first.
pub fn get(name: &str) -> Arc<Net> {
    let mut nets = registry().lock().expect("the net registry is not poisoned");
    if let Some(net) = nets.get(name) {
        return net.clone();
    }
    let bytes = embedded(name).unwrap_or_else(|| panic!("the net {name} is not loaded"));
    let net = Arc::new(Net::from_bytes(bytes).expect("the built-in weights are valid"));
    nets.insert(name.to_string(), net.clone());
    net
}
