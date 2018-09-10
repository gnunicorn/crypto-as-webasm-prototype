use wasm_bindgen::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Window, Crypto}; 
use futures::Future;

use multihash::Multihash;
use wasm_bindgen::JsCast;
use js_sys::{ArrayBuffer, Uint8Array};


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hash(input: &str) {
	let crypto : Crypto = Window::crypto().unwrap();
	let fut = JsFuture::from(crypto.subtle()
			.digest_with_str_and_u8_array("SHA-256", input.as_ref()).unwrap());
	fut.and_then(|res| {
		let buf = res.dyn_into::<Uint8Array>().unwrap();
		alert(&format!("{:?}", buf));
		Ok(())
	});
}