# linked-list-rs

A custom implementation of a **doubly linked list** in Rust using `unsafe` code for manual memory management. This project serves as an educational example of how linked lists can be implemented in Rust without relying on the standard library's collections.

## Features

- **Doubly Linked List**: Supports bidirectional traversal.
- **Safe API with Unsafe Internals**: Exposes a safe, idiomatic Rust API while using `unsafe` internally for pointer manipulation.
- **Efficient Insertions and Deletions**: Provides `O(1)` complexity for insertions and deletions at arbitrary positions.
- **Iterator Support**: Implements standard Rust iterators for easy traversal.
- **Cursor & CursorMut**: Provides fine-grained control over list navigation and modification.

## Installation

To use this linked list in your Rust project, add it as a dependency:

```toml
[dependencies]
linked-list-rs = { git = "https://github.com/dmitriiantonov/linked-list-rs" }
```

## Usage

### Creating a List

```rust
use linked_list_rs::LinkedList;

fn main() {
    let mut list = LinkedList::new();

    list.push_front(1);
    list.push_back(2);
    list.push_back(3);

    println!("List length: {}", list.len());
}
```

### Iterating Over Elements

```rust
for value in list.iter() {
    println!("{}", value);
}
```

### Using Cursors

```rust
use linked_list_rs::{LinkedList, CursorMut};

let mut list = LinkedList::new();
list.push_back(1);
list.push_back(2);
list.push_back(3);

let mut cursor = list.cursor_mut();
cursor.move_next();
cursor.insert_after(4);
```

### Removing Elements

```rust
list.pop_front(); // Removes the first element
list.pop_back();  // Removes the last element
```

## API Overview

### Methods

| Method               | Description |
|----------------------|-------------|
| `LinkedList::new()`  | Creates an empty linked list. |
| `push_front(value)`  | Inserts a value at the front. |
| `push_back(value)`   | Inserts a value at the back. |
| `pop_front()`        | Removes and returns the front element. |
| `pop_back()`         | Removes and returns the back element. |
| `iter()`             | Returns an iterator over the list. |
| `len()`              | Returns the number of elements in the list. |
| `is_empty()`         | Checks if the list is empty. |
| `cursor()`           | Returns an immutable cursor for traversal. |
| `cursor_mut()`       | Returns a mutable cursor for modification. |

## Safety Considerations

This implementation uses **raw pointers and manual memory management** (`unsafe` Rust). While the API is designed to be safe, improper modifications could lead to **memory leaks or undefined behavior**. If you modify the internals, ensure proper pointer updates and deallocations.

## Contributing

Contributions are welcome! If you find a bug or want to propose an enhancement, feel free to open an issue or submit a pull request.

## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for more details.

