pub struct BuddyAllocator{

}


impl BuddyAllocator{
    pub fn new(memory: *mut u8, size: usize) -> Self;

    pub fn alloc(&mut self, size: usize) -> Option<*mut u8>;

    pub fn dealloc(&mut self, ptr: *mut u8, size: usize);

    pub fn dump(&self);
}
