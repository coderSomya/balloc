pub fn is_power_of_two(x: usize) -> bool {
    x != 0 && (x & (x - 1)) == 0
}

pub fn next_power_of_two(x: usize) -> usize {
    x.next_power_of_two()
}

pub fn log2(x: usize) -> usize {
    usize::BITS as usize - 1 - x.leading_zeros() as usize
}

pub fn order_to_size(min_block_size: usize, order: usize) -> usize {
    min_block_size << order
}

pub fn size_to_order(min_block_size: usize, size: usize) -> usize {
    let rounded = next_power_of_two(size.max(min_block_size));
    log2(rounded) - log2(min_block_size)
}
