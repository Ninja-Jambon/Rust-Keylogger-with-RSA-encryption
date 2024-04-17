use rand::Rng;
use num_integer::gcd;

fn is_prime(n: u64) -> bool {
	if n < 2 {
		return false;
	}

	let mut i = 2;

	while i < (n as f64).sqrt() as u64 + 1 {
		if n % i == 0 {
			return false;
		}
		i+=1;
	}

	return true;
}

fn gen_prime(a: u64, b: u64) -> u64 {
	let mut rng = rand::thread_rng();
	let mut n = rng.gen_range(a..b);

	while !is_prime(n) {
		n = rng.gen_range(a..b);
	}

	return n;
}

fn mod_inverse(e: u64, phi: u64) -> u64 {
	let mut d: u64 = 3;

	while d < phi {
		if (d * e) % phi == 1 {
			return d
		}
		d+=1;
	}

	panic!("Error");
}

pub fn gen_keys(a: u64, b: u64) -> (u64, u64, u64) {
	let p = gen_prime(a, b);
	let mut q = gen_prime(a, b);

	while p == q {
		q = gen_prime(a, b);
	}

	let n = p * q;
	let phi_n = (p - 1) * (q - 1);

	let mut rng = rand::thread_rng();
	let mut e = rng.gen_range(3..(phi_n-1));

	while gcd(e, phi_n) != 1 {
		e = rng.gen_range(3..phi_n-1);
	}

	let d = mod_inverse(e, phi_n);

	(e, d, n)
}

fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
	if modulus == 1 { return 0 }
	let mut result = 1;
	base = base % modulus;
	while exp > 0 {
		if exp % 2 == 1 {
			result = result * base % modulus;
		}
		exp = exp >> 1;
		base = base * base % modulus
	}
	return result
}

pub fn array_mod_pow(bytes: &[u64], d: u64, n: u64) -> Vec<u64> {
    let mut new_bytes = vec![0u64; bytes.len()];

    let mut i = 0;

    while i < bytes.len() {
    	new_bytes[i] = mod_pow(bytes[i] as u64, d, n);
    	i+=1;
    }

    return new_bytes;
}

/*
fn main() {
	let (e, d, n) = gen_keys(10000, 50000);

	println!("e : {}", e);
	println!("d : {}", d);
	println!("n : {}", n);

	let message = "Hello, world!";
    let message_as_64: Vec<u64> = message
        .as_bytes()
        .iter()
        .map(|&x| x as u64)
        .collect();

    println!("{:?}", message_as_64);
    println!(
        "{:?}",
        array_mod_pow(&array_mod_pow(&message_as_64, e, n), d, n)
    );
}
*/
