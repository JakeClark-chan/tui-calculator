pub struct Calculator {
    display: String,
    previous_value: f64,
    operation: Option<Operation>,
    waiting_for_new_number: bool,
    error: Option<Error>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}
#[derive(Debug)]
pub enum Error {
    DivisionByZero,
    UndefinedOperation,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            display: String::from("0"),
            previous_value: 0.0,
            operation: None,
            waiting_for_new_number: false,
            error: None,
        }
    }

    pub fn display(&self) -> &str {
        match &self.error {
            Some(Error::DivisionByZero) => "DivisionByZero",
            Some(Error::UndefinedOperation) => "UndefinedOperation",
            None => &self.display,
        }
    }

    pub fn clear(&mut self) {
        self.display = "0".to_string();
        self.previous_value = 0.0;
        self.operation = None;
        self.waiting_for_new_number = false;
        self.error = None;
    }

    pub fn input_digit(&mut self, num: u8) {
        // Check if waiting for new number (after press operation) -> clear and immediately write new number, and no need to wait for new number
        if self.waiting_for_new_number {
            self.display = "".to_string();
            self.waiting_for_new_number = false;
        }
        // Clear screen if it has "0"
        if self.display == "0" {
            // Clear display
            self.display = num.to_string();
        } else {
            // Append new value
            self.display.push_str(&num.to_string());
        }
    }

    pub fn get_operation_status(&self) -> Option<Operation> {
        self.operation
    }

    pub fn set_operation(&mut self, op: Operation) {
        // If has op: Calculate first
        if self.operation.is_some() {
            self.calculate();
        }
        // Always update: current number, new operation, despite calculate operation (because inside calculate, operation set to None)
        self.previous_value = self.display.parse().unwrap_or(0.0);
        self.operation = Some(op);
        self.waiting_for_new_number = true;
    }

    pub fn calculate(&mut self) {
        // dbg!(&self.operation);
        // Get current screen number
        let num: f64 = self.display.parse().unwrap_or(0.0);
        let mut result: f64 = self.previous_value;
        // Do operation
        match self.operation {
            Some(Operation::Add) => result += num,
            Some(Operation::Subtract) => result -= num,
            Some(Operation::Multiply) => result *= num,
            Some(Operation::Divide) => {
                // Can't divide by zero - we will make flag for now
                if num == 0. {
                    self.error = Some(Error::DivisionByZero)
                } else {
                    result /= num;
                }
            }
            None => self.error = Some(Error::UndefinedOperation),
        }
        // Delete display, update answer to screen and delete operation
        self.display = result.to_string();
        self.operation = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_display_is_zero() {
        let calc = Calculator::new();
        assert_eq!(calc.display(), "0");
    }

    #[test]
    fn test_input_digits() {
        let mut calc = Calculator::new();
        calc.input_digit(5);
        assert_eq!(calc.display(), "5");
        calc.input_digit(2);
        assert_eq!(calc.display(), "52");
    }

    #[test]
    fn test_addition() {
        let mut calc = Calculator::new();
        calc.input_digit(1);
        calc.set_operation(Operation::Add);
        calc.input_digit(2);
        calc.calculate();
        assert_eq!(calc.display(), "3");
    }

    #[test]
    fn test_chained_addition() {
        let mut calc = Calculator::new();
        calc.input_digit(1);
        calc.set_operation(Operation::Add);
        calc.input_digit(2);
        calc.set_operation(Operation::Add); // Lúc này nó nên tính ra 3 và hiển thị 3
        assert_eq!(calc.display(), "3");
        calc.input_digit(3);
        calc.calculate();
        assert_eq!(calc.display(), "6");
    }

    #[test]
    fn test_divide_by_zero_shows_error() {
        let mut calc = Calculator::new();
        calc.input_digit(5);
        calc.set_operation(Operation::Divide);
        calc.input_digit(0);
        calc.calculate();
        assert_eq!(calc.display(), "DivisionByZero");
    }

    // Clear
    #[test]
    fn test_clear() {
        let mut calc = Calculator::new();
        calc.input_digit(5);
        calc.clear();
        assert_eq!(calc.display(), "0");
    }
}
