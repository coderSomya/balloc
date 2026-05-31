use crate::freelist::*;
use crate::utils::*;

use core::mem;
use core::ptr::NonNull;

#[repr(C)]
struct AllocationHeader {
    order: u8,
}

pub struct BuddyAllocator<'a> {
    memory: &'a mut [u8],
    base: *mut u8,

    min_block_size: usize,
    max_order: usize,

    free_lists: Vec<Option<NonNull<FreeBlock>>>,
}

impl<'a> BuddyAllocator<'a> {
    pub fn new(
        memory: &'a mut [u8],
        min_block_size: usize,
    ) -> Self {
        assert!(is_power_of_two(memory.len()));
        assert!(is_power_of_two(min_block_size));

        let max_order =
            log2(memory.len()) - log2(min_block_size);

        let mut free_lists =
            vec![None; max_order + 1];

        let base = memory.as_mut_ptr();

        unsafe {
            let block = base as *mut FreeBlock;

            push(
                &mut free_lists[max_order],
                block,
            );
        }

        Self {
            memory,
            base,
            min_block_size,
            max_order,
            free_lists,
        }
    }

    fn offset_of(&self, ptr: *mut u8) -> usize {
        ptr as usize - self.base as usize
    }

    fn ptr_at(&self, offset: usize) -> *mut u8 {
        unsafe { self.base.add(offset) }
    }

    pub fn alloc(
        &mut self,
        requested: usize,
    ) -> Option<NonNull<u8>> {
        let total =
            requested + mem::size_of::<AllocationHeader>();

        let target_order =
            size_to_order(self.min_block_size, total);

        if target_order > self.max_order {
            return None;
        }

        let mut found_order = target_order;

        while found_order <= self.max_order {
            if self.free_lists[found_order].is_some() {
                break;
            }

            found_order += 1;
        }

        if found_order > self.max_order {
            return None;
        }

        let block_ptr = unsafe {
            pop(&mut self.free_lists[found_order])?
        };

        let mut offset =
            self.offset_of(block_ptr as *mut u8);

        while found_order > target_order {
            found_order -= 1;

            let size =
                order_to_size(
                    self.min_block_size,
                    found_order,
                );

            let buddy_offset = offset + size;

            unsafe {
                let buddy =
                    self.ptr_at(buddy_offset)
                        as *mut FreeBlock;

                push(
                    &mut self.free_lists[found_order],
                    buddy,
                );
            }
        }

        unsafe {
            let header =
                self.ptr_at(offset)
                    as *mut AllocationHeader;

            (*header).order =
                target_order as u8;

            let user_ptr =
                (header as *mut u8)
                    .add(mem::size_of::<AllocationHeader>());

            Some(
                NonNull::new_unchecked(user_ptr)
            )
        }
    }

    pub fn dealloc(
        &mut self,
        ptr: NonNull<u8>,
    ) {
        unsafe {
            let header_ptr =
                ptr.as_ptr()
                    .sub(mem::size_of::<AllocationHeader>())
                    as *mut AllocationHeader;

            let mut order =
                (*header_ptr).order as usize;

            let mut offset =
                self.offset_of(header_ptr as *mut u8);

            while order < self.max_order {
                let block_size =
                    order_to_size(
                        self.min_block_size,
                        order,
                    );

                let buddy_offset =
                    offset ^ block_size;

                let buddy_ptr =
                    self.ptr_at(buddy_offset)
                        as *mut FreeBlock;

                let removed =
                    remove(
                        &mut self.free_lists[order],
                        buddy_ptr,
                    );

                if !removed {
                    break;
                }

                offset =
                    offset.min(buddy_offset);

                order += 1;
            }

            let merged =
                self.ptr_at(offset)
                    as *mut FreeBlock;

            push(
                &mut self.free_lists[order],
                merged,
            );
        }
    }

    pub fn dump(&self) {

        for order in 0..=self.max_order {
            let size =
                order_to_size(
                    self.min_block_size,
                    order,
                );

            print!(
                "order={} size={} : ",
                order,
                size
            );

            let mut current =
                self.free_lists[order];

            while let Some(node) = current {
                unsafe {
                    let ptr =
                        node.as_ptr()
                            as *mut u8;

                    let offset =
                        ptr as usize
                            - self.base as usize;

                    print!("[{}] ", offset);

                    current =
                        (*node.as_ptr()).next;
                }
            }

            println!();
        }
    }

    pub fn total_size(&self) -> usize {
        self.memory.len()
    }
}
