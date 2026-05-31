## Buddy Allocator

[Paper](https://arxiv.org/pdf/1804.03436)

### Example Usage

```rust
use buddy_allocator::BuddyAllocator;

fn main() {
    let mut heap = vec![0u8; 1024];

    let mut alloc =
        BuddyAllocator::new(&mut heap, 16);

    alloc.dump();

    let a = alloc.alloc(100).unwrap();
    let b = alloc.alloc(200).unwrap();
    let c = alloc.alloc(50).unwrap();

    println!("\nAfter allocs");
    alloc.dump();

    alloc.dealloc(a);

    println!("\nAfter free A");
    alloc.dump();

    alloc.dealloc(b);

    println!("\nAfter free B");
    alloc.dump();

    alloc.dealloc(c);

    println!("\nAfter free C");
    alloc.dump();
}
```


