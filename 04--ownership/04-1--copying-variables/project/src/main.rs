/*

Multiple variables can interact with the same data in different ways in Rust.

```
$ cd 04*
$ cd 04-1*
$ cargo new project
$ cd project
```
 */

fn main() {
    let x = 5;
    let y = x;
    // We now have two variables, `x` and `y`, and both equal 5.

    let s1 = String::from("hello");
    /*

    A `String` is made up of three parts:
    - a pointer to the memory that holds the contents of the string,
    - a length = the memory of the contents in bytes, and
    - a capacity = the total amount of memory in bytes received from the allocator.

    This group of data is stored on the stack.

    | name     | value                                |
    |----------|--------------------------------------|
    | ptr      | <pointer to the index 0 on the heap> |
    | len      | 5                                    |
    | capacity | 5                                    |

    The contents are stored on the heap.

    | index | value |
    |-------|-------|
    | 0     | `h`   |
    | 1     | `e`   |
    | 2     | `l`   |
    | 3     | `l`   |
    | 4     | `o`   |

     */

    // Copy the pointer, the length, and the capacity to the memory on the stack, and
    // don't copy the data on the heap that the pointer refers to.
    let s2 = s1;

}
