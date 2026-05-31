## Buddy Allocator

[Paper](https://arxiv.org/pdf/1804.03436)

### Example Usage

```rust
let mut heap = vec![0u8; 1024];

let mut alloc = BuddyAllocator::new(
    heap.as_mut_ptr(),
    heap.len(),
);

let p1 = alloc.alloc(100).unwrap();
let p2 = alloc.alloc(200).unwrap();

alloc.dealloc(p1, 100);
alloc.dealloc(p2, 200);
```


