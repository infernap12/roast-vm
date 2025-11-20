use std::cell::Cell;
use std::sync::atomic::{AtomicU32, Ordering};

static RAND_SEED: AtomicU32 = AtomicU32::new(1234567);

fn next_random(rand_seed: u32) -> u32 {
	const A: u32 = 16807;
	const M: u32 = 2147483647; // 2^31 - 1

	// compute az=2^31p+q
	let mut lo = A * (rand_seed & 0xFFFF);
	let hi = A * (rand_seed >> 16);
	lo += (hi & 0x7FFF) << 16;

	// if q overflowed, ignore the overflow and increment q
	if lo > M {
		lo &= M;
		lo += 1;
	}
	lo += hi >> 15;

	// if (p+q) overflowed, ignore the overflow and increment (p+q)
	if lo > M {
		lo &= M;
		lo += 1;
	}
	lo
}

fn os_random() -> u32 {
	loop {
		let seed = RAND_SEED.load(Ordering::Relaxed);
		let rand = next_random(seed);
		match RAND_SEED.compare_exchange(seed, rand, Ordering::Relaxed, Ordering::Relaxed) {
			Ok(_) => return rand,
			Err(_) => continue, // Another thread updated seed, retry
		}
	}
}

thread_local! {
    static XORSHIFT_STATE: Cell<[u32; 4]> = {
        Cell::new([
            os_random(),  // X: seeded from global Park-Miller RNG
            842502087,    // Y: constant
            0x8767,       // Z: constant
            273326509,    // W: constant
        ])
    };
}

pub fn generate_identity_hash() -> u32 {
	XORSHIFT_STATE.with(|state| {
		let mut s = state.get();

		let mut t = s[0];
		t ^= t << 11;
		s[0] = s[1];
		s[1] = s[2];
		s[2] = s[3];

		let mut v = s[3];
		v = (v ^ (v >> 19)) ^ (t ^ (t >> 8));
		s[3] = v;

		state.set(s);
		v
	})
}