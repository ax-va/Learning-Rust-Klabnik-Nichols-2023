pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn pop(&mut self) -> Option<i32> {
        let result = self.list.pop();
        if result.is_some() {
            self.update_average();
        }
        result
    }

    pub fn average(&self) -> Option<f64> {
        if self.list.is_empty() {
            None
        } else {
            Some(self.average)
        }
    }

    fn update_average(&mut self) {
        if !self.list.is_empty() {
            let total: i32 = self.list.iter().sum();
            // We can modify the `average` field inside `update_average`
            // because methods in the `impl` of a `struct` always have unrestricted access to all its fields,
            // even if they're not `pub`.
            self.average = total as f64 / self.list.len() as f64;
        }
    }
}

fn main() {
    println!("Hello, world!");
}
