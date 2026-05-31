use core::ptr::NonNull;

#[repr(C)]
pub struct FreeBlock {
    pub next: Option<NonNull<FreeBlock>>,
}

pub unsafe fn push(
    head: &mut Option<NonNull<FreeBlock>>,
    block: *mut FreeBlock,
) {
    unsafe {
        (*block).next = *head;
        *head = Some(NonNull::new_unchecked(block));
    }
}

pub unsafe fn pop(
    head: &mut Option<NonNull<FreeBlock>>,
) -> Option<*mut FreeBlock> {
    unsafe {
        let current = head.take()?;

        let ptr = current.as_ptr();

        *head = (*ptr).next;

        Some(ptr)
    }
}

pub unsafe fn remove(
    head: &mut Option<NonNull<FreeBlock>>,
    target: *mut FreeBlock,
) -> bool {
    unsafe {
        let mut current = *head;
        let mut prev: Option<NonNull<FreeBlock>> = None;

        while let Some(node) = current {
            let ptr = node.as_ptr();

            if ptr == target {
                let next = (*ptr).next;

                match prev {
                    None => *head = next,
                    Some(prev_node) => {
                        (*prev_node.as_ptr()).next = next;
                    }
                }

                return true;
            }

            prev = current;
            current = (*ptr).next;
        }

        false
    }
}
