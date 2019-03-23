#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate canonical_json;
extern crate spectral;

pub mod cnab;

#[cfg(test)]
mod tests;
