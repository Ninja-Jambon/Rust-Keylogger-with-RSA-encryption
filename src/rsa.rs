use num_bigint_dig::{ToBigUint, RandBigInt, BigUint};
use num_bigint_dig::traits::ModInverse;

fn get_s_and_d(n: &BigUint) -> (BigUint, BigUint) {
	let mut d = n - 1.to_biguint().unwrap();
	let mut s = 0.to_biguint().unwrap();

	while &d % 2.to_biguint().unwrap() == 0.to_biguint().unwrap() {
		s += 1.to_biguint().unwrap();
		d /= 2.to_biguint().unwrap();
	}

	(d, s)
}

fn miller_witness(n: &BigUint, a: &BigUint) -> bool {
	let (d, s) = get_s_and_d(&n);

	let mut x = a.modpow(&d, &n);

	if x == 1.to_biguint().unwrap() {
		return false;
	}

	let mut i = 0.to_biguint().unwrap();
	while i < s.clone(){
		if x == n.clone() - 1.to_biguint().unwrap() {
			return false;
		}

		x = x.modpow(&2.to_biguint().unwrap(), &n);

		i += 1.to_biguint().unwrap();
	}

	true
}

fn is_prime(n: BigUint, k: u16) -> bool {

	if n.clone() == 3.to_biguint().unwrap() || n.clone() == 2.to_biguint().unwrap() {
		return true;
	}
	if n.clone() == 1.to_biguint().unwrap() {
		return false;
	}
	if n.clone() % 2.to_biguint().unwrap() == 0.to_biguint().unwrap() {
		return false;
	}

	for _ in 0..k {
		let mut rng = rand::thread_rng();
		let min = 2.to_biguint().unwrap();
		let max = n.clone() - 1.to_biguint().unwrap();
		let a = rng.gen_biguint_range(&min, &max);

		if miller_witness(&n, &a) {
			return false;
		}
	}

	return true;
}

fn gen_prime_number(lenght: usize) -> BigUint {
	let mut rng = rand::thread_rng();

	let mut a = rng.gen_biguint(lenght);

	while !is_prime(a.clone(), 50) {
		a = rng.gen_biguint(lenght);
	}

	a
}

pub fn gen_keys(lenght: usize) -> (BigUint, BigUint, BigUint) {
	let p = gen_prime_number(lenght / 2);
	let mut q = gen_prime_number(lenght / 2);

	while p == q {
		q = gen_prime_number(lenght / 2);
	}

	let n = &p * &q;
	let phi_n = (&p - 1.to_biguint().unwrap()) * (&q - 1.to_biguint().unwrap());
	let e = 65537.to_biguint().unwrap();
	let d = e.clone().mod_inverse(&phi_n).unwrap().to_biguint().expect("error");

	(e, d, n)
}

pub fn pad_data(data: Vec<u8>, length: usize) -> Vec<u8> {
	let mut padded_data = data;

	let num_zeros = (length / 8) - padded_data.len();

	for _ in 0..num_zeros {
		padded_data.insert(0, 0);
	}

	padded_data
}