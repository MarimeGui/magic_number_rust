# Magic Number

Simply checks for a specific Magic Number read by anything that implements a Read trait.

Example:

```rust
extern crate magic_number;

use magic_number::check_magic_number;

fn main() {
    let ref mut reader = ...;
        check_magic_number(reader, vec![b'T', b'E', b'S', b'T']).unwrap();
}
```