/*
*Associated types* let a trait declare a type as a placeholder
that trait methods can use in their signatures
and that each implementation must concretely define.

- A trait can declare an associated type as a placeholder.
- Methods in the trait can use that type in their signatures.
- Each implementer chooses *one concrete type*, fixed for that implementation.

Difference between associated types and generic parameters:
- Associated types define one concrete type per trait implementation,
while generics allow a trait to be implemented multiple times
for the same type with different type parameters.
 */

// Example: trait with an associated type

trait ContainerAT {
    type Item; // associated type (placeholder)

    fn add(&mut self, value: Self::Item);
    fn get_all(&self) -> &[Self::Item];
    // `&[X]` is a slice of `X` - a reference to a sequence of `X` values
}

struct IntContainer {
    data: Vec<i32>,
}

impl ContainerAT for IntContainer {
    type Item = i32; // concrete type

    fn add(&mut self, value: i32) {
        self.data.push(value);
    }

    fn get_all(&self) -> &[i32] {
        &self.data
    }
}

// Comparison: trait with a generic parameter

trait ContainerG<T> {
    fn add(&mut self, value: T);
    fn get_all(&self) -> &[T];
}

struct GenericContainer<T> {
    data: Vec<T>,
}

impl ContainerG<i32> for GenericContainer<i32> {
    fn add(&mut self, value: i32) {
        self.data.push(value);
    }

    fn get_all(&self) -> &[i32] {
        &self.data
    }
}

impl ContainerG<f64> for GenericContainer<f64> {
    fn add(&mut self, value: f64) {
        self.data.push(value);
    }

    fn get_all(&self) -> &[f64] {
        &self.data
    }
}

fn main() {
    // associated type container (only i32)
    let mut int_container = IntContainer { data: Vec::new() };
    int_container.add(10);
    int_container.add(20);

    println!("Associated type container: {:?}", int_container.get_all());
    // Associated type container: [10, 20]

    // generic container for i32
    let mut gc_i32 = GenericContainer { data: Vec::new() };
    gc_i32.add(1);
    gc_i32.add(2);

    println!("Generic container i32: {:?}", gc_i32.get_all());
    // Generic container i32: [1, 2]

    // generic container for f64
    let mut gc_f64 = GenericContainer { data: Vec::new() };
    gc_f64.add(1.5);
    gc_f64.add(2.5);

    println!("Generic container f64: {:?}", gc_f64.get_all());
    // Generic container f64: [1.5, 2.5]
}
