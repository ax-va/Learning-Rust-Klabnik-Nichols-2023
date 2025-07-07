/*
```
$ cd 12*
$ cd 12-8*
$ cargo new project --lib
$ cd project
$ cargo test
...
running 1 test
test tests::filters_by_size ... ok
...
```

*Iterator adapters*
transform or extend an existing iterator to produce a new iterator.
They are *lazy*, meaning they don't do anything until the iterator is actually consumed.

Iterator adaptors:
- Take a reference (often `&mut self`) to the iterator;
- Return a new iterator, usually lazy (it doesn't run `next` until we iterate it).
- Can call `next` internally, but they don't consume the whole iterator themselves.
- Can take closures as arguments that capture their environment.
- Examples: `map`, `filter`, `peekable`, `enumerate`.
 */

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }
}
