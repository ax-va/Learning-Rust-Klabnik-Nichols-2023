pub struct AveragedCollection {
    list: Vec<i32>, // private field
    average: f64, // private field
}

impl AveragedCollection {
    pub fn new() -> Self {
        Self { list: Vec::new(), average: 0.0 }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn pop(&mut self) -> Option<i32> {
        let result = self.list.pop();
        self.update_average();
        result
    }

    // public API only to read the average but not modify it
    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let len = self.list.len();
        // We can modify the `average` field inside `update_average`
        // because methods in the `impl` of a `struct` always have unrestricted access to all its fields,
        // even if they're not `pub`.
        self.average = if len == 0 {
            0.0
        } else {
            let total: i32 = self.list.iter().sum();
            total as f64 / len as f64
        };
    }
}

fn main() {
    // `add` and `pop` uses `&mut self`
    let mut collection = AveragedCollection::new();
    collection.add(1);
    collection.add(2);
    collection.add(3);
    println!("Average: {:?}", collection.average());
    // Average: 2.0

    let removed = collection.pop();
    println!("Removed: {:?}", removed);
    // Removed: Some(3)
    println!("Average: {:?}", collection.average());
    // Average: 1.5
}