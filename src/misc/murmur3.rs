pub trait MurmurHash {
    fn murmur(&self) -> u32;
}


#[inline(always)]
pub fn murmur_init(seed: u32) -> u32 {
    seed
}

#[inline]
pub fn murmur_update(h: u32, v: u32) -> u32 {
    const C1: u32 = 0xcc9e2d51;
    const C2: u32 = 0x1b873593;
    const R1: u32 = 15;
    const R2: u32 = 13;
    const M: u32 = 5;
    const N: u32 = 0xe6546b64;

    let mut k = v;
    k = k.wrapping_mul(C1);
    k = k.wrapping_shl(R1) | (k.wrapping_shr(32 - R1));
    k = k.wrapping_mul(C2);

    let mut hash = h ^ k;
    hash = hash.wrapping_shl(R2) | (hash.wrapping_shr(32 - R2));
    hash = hash.wrapping_mul(M).wrapping_add(N);
    hash
}

#[inline]
pub fn murmur_finish(h: u32, num: u32) -> u32 {
    let mut hash = h;
    hash ^= num.wrapping_shl(2);
    hash ^= hash.wrapping_shr(16);
    hash = hash.wrapping_mul(0x85ebca6b);
    hash ^= hash.wrapping_shr(13);
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash ^= hash.wrapping_shr(16);
    hash
}