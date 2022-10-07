#[inline(always)]
pub fn isqrt(x: usize) -> usize {
    (x as f64).sqrt() as usize
}

// 0 -> 1
// 1 -> 3
// 2 -> 5
// ......
#[inline(always)]
pub fn to_index(x: usize) -> usize {
    (x - 1) / 2
}
#[inline(always)]
pub fn from_index(x: usize) -> usize {
    x * 2 + 1
}
