extern crate multihash;

extern crate futures;
extern crate activitystreams_types;

#[cfg(target_arch="wasm32")]
extern crate web_sys;
#[cfg(target_arch="wasm32")]
extern crate js_sys;
#[cfg(target_arch="wasm32")]
extern crate wasm_bindgen_futures;
#[cfg(target_arch="wasm32")]
extern crate wasm_bindgen;


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod as_types;
mod protocol;
#[cfg(target_arch="wasm32")]
mod wasm_interface;