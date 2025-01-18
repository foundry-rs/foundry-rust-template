#[allow(warnings)]
mod bindings;
use bindings::Guest;

struct Component;

impl Guest for Component {
    fn process_eth_trigger(input: Vec<u8>) -> std::result::Result<Vec<u8>, String> {
        Ok(input)
    }
}

bindings::export!(Component with_types_in bindings);
