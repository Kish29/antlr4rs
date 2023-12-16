use std::hash::{Hash, Hasher};
use criterion::{black_box, Criterion, criterion_group, criterion_main};
use antlr4rs::misc::murmur3::{murmur_finish, murmur_init, murmur_update};

struct ShouldHash {
    vecs: Vec<UseMurmur>,
}

impl ShouldHash {
    fn hash_code(&self) -> u32 {
        let mut h: u32 = 1;
        for ue in &self.vecs {
            h = h.wrapping_mul(31).wrapping_add(ue.hash_code());
        }
        h
    }
}

struct UseMurmur {
    s1: u32,
    s2: u32,
    c: u32,
    d: u32,
}

impl UseMurmur {
    pub fn hash_code(&self) -> u32 {
        let mut h = murmur_init(7);
        h = murmur_update(h, self.s1);
        h = murmur_update(h, self.s2);
        h = murmur_update(h, self.c);
        h = murmur_update(h, self.d);
        murmur_finish(h, 4)
    }
}

#[test]
fn test_hasher() {
    let mut sh = ShouldHash { vecs: vec![] };
    sh.vecs.push(UseMurmur { s1: 1, s2: 2, c: 3, d: 4 });
    sh.vecs.push(UseMurmur { s1: 1, s2: 2, c: 3, d: 4 });
    sh.vecs.push(UseMurmur { s1: 1, s2: 2, c: 3, d: 4 });
    sh.vecs.push(UseMurmur { s1: 1, s2: 2, c: 3, d: 4 });
    sh.vecs.push(UseMurmur { s1: 1, s2: 2, c: 3, d: 4 });
    sh.vecs.push(UseMurmur { s1: 1, s2: 2, c: 3, d: 4 });
    println!("{}", sh.hash_code());
}

fn gen_hash() {
    let mut sh = ShouldHash { vecs: vec![] };
    sh.vecs.push(UseMurmur { s1: 1, s2: 2, c: 3, d: 4 });
    sh.vecs.push(UseMurmur { s1: 1, s2: 2, c: 3, d: 4 });
    sh.vecs.push(UseMurmur { s1: 1, s2: 2, c: 3, d: 4 });
    sh.vecs.push(UseMurmur { s1: 1, s2: 2, c: 3, d: 4 });
    sh.vecs.push(UseMurmur { s1: 1, s2: 2, c: 3, d: 4 });
    sh.vecs.push(UseMurmur { s1: 1, s2: 2, c: 3, d: 4 });
    // println!("{}", sh.hash_code());
}

fn bench_hash(c: &mut Criterion) {
    c.bench_function("murmur3 hash", |b| b.iter(|| {
        black_box(gen_hash())
    }));
}


criterion_group!(benches, bench_hash);
criterion_main!(benches);
