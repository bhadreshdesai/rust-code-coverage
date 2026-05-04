// Explicit return using 'return' keyword and semicolon
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

// Implicit return (no semicolon at the end)
fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn main() {
    let x = 10;
    let y = 3;
    let sum = add(x, y);
    let difference = subtract(x, y);

    println!("{} + {} = {}", x, y, sum);        // Output: Sum: 15
    println!("{} - {} = {}", x, y, difference); // Output: Difference: 5
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dummy_test() {
        assert!(true);
    }

    #[test]
    fn test_add() {
        assert_eq!(add(3, 2), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(3, 2), 1);
    }
}