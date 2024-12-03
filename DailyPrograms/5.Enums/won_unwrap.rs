
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

impl SpreadsheetCell {
    // Custom unwrap for `Int` variant
    pub fn unwrap_int(self) -> i32 {
        if let SpreadsheetCell::Int(value) = self {
            value
        } else {
            panic!("Called `unwrap_int` on a non-Int variant!");
        }
    }

    // Custom unwrap for `Float` variant
    pub fn unwrap_float(self) -> f64 {
        if let SpreadsheetCell::Float(value) = self {
            value
        } else {
            panic!("Called `unwrap_float` on a non-Float variant!");
        }
    }

    // Custom unwrap for `Text` variant
    pub fn unwrap_text(self) -> String {
        if let SpreadsheetCell::Text(value) = self {
            value
        } else {
            panic!("Called `unwrap_text` on a non-Text variant!");
        }
    }
}

fn main() {
    let cell = SpreadsheetCell::Int(42);

    // Using the custom unwrap method
    let value = cell.unwrap_int();
    println!("The value is: {}", value);
}

