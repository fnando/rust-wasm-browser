use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two_numbers() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn greets_jane() {
        let result = greet("Jane");
        assert_eq!(result, "Hello, Jane!");
    }
}
