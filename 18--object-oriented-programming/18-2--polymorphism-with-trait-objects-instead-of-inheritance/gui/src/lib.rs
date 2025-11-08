// A *trait* in Rust is like an *interface* in other languages:
// - It defines behavior that types can implement.
// - Any type that wants to be "drawable" must implement `fn draw(&self)`.
pub trait Draw {
    fn draw(&self);
}

// `Box<dyn Draw>`:
// - `dyn Draw` means a *trait object*:
// a value whose concrete type is unknown at compile time
// but which implements the `Draw` trait.
// - `Box<dyn Draw>` means allocating that trait object on the heap and storing a pointer to it.

// As a result:
// - Rust does not know the exact type inside the `Box` at compile time.
// - But it does know it implements `Draw`, so it can call `draw` on it at runtime.

// Vectors:
// - A vector needs to know the size of its elements.
// - Trait objects (`dyn Draw`) have no known size.
// - `Box<dyn Draw>` has a known size (a pointer), so it works.

// Trait bounds vs. trait objects:
// - Trait bounds give us *compile-time polymorphism* with one concrete type at a time
// -> *static dispatch*, which is when the compiler knows what method you are calling at compile time.
// - Trait objects give us *runtime polymorphism* and let us mix different types behind a shared trait
// -> *dynamic dispatch*, which is when the compiler cannot tell at compile time which method we are calling.

// *Polymorphism* lets us treat different drawable components uniformly and call `draw` on all of them

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!(
            "A button is drawn with width = {:?}, height = {:?}, label = {:?}",
            self.width, self.height, self.label
        )
    }
}
