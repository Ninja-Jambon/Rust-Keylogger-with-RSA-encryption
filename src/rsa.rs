use rand;
use rand::Rng;
use num::{BigInt, One, Pow, ToPrimitive, Zero};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use primal::Primes;

const EXPONENT: i64 = 65537;

fn puissance_modulaire(base: &BigUint, exposant: &BigUint, n: &BigUint) -> BigUint {
	base.modpow(&exposant, &n)
}

fn generer_cle(bits: u32) -> (BigUint, BigUint, BigUint) {
	let un = BigUint::one();
	let deux = BigUint::from(2u32);

	let p = generer_premier(bits);
	let q = generer_premier(bits);

	let n = &p * &q;
	let totient = (&p - &un) * (&q - &un);

	let mut exposant_public = deux;
	while (&exposant_public * &totient - &un).is_zero() || ((&exposant_public * &totient - &un).to_i64().unwrap() % EXPONENT == 0) {
		exposant_public += un;
	}

	let mut cle_privee = exposant_public.inverse(&totient).unwrap();

	(n, cle_privee, exposant_public)
}

fn generer_premier(bits: u32) -> BigUint {
	let mut generateur_aleatoire = rand::thread_rng();
	let mut candidat_premier: BigUint;

	loop {
		candidat_premier = generateur_aleatoire.gen_biguint(bits as usize);
		if Primes::all().any(|p| candidat_premier == p) {
			break;
		}
	}

	candidat_premier
}


fn chiffrer_public(message: &[u8], exposant_public: &BigUint, n: &BigUint) -> Vec<u8> {
	message.iter()
		.map(|octet| puissance_modulaire(&BigUint::from_u32(*octet as u32).unwrap(), exposant_public, n))
		.map(|ciphertext| ciphertext.to_u32().unwrap() as u8)
		.collect()
}

fn dechiffrer_prive(ciphertext: &[u8], cle_privee: &BigUint, n: &BigUint) -> Vec<u8> {
	ciphertext.iter()
		.map(|octet| puissance_modulaire(&BigUint::from_u32(*octet as u32).unwrap(), cle_privee, n))
		.map(|plaintext| plaintext.to_u32().unwrap() as u8)
		.collect()
}

fn main() {
	let (n, cle_privee, exposant_public) = generer_cle(1024);
	let message = b"Hello, world!";

	let ciphertext = chiffrer_public(message, &exposant_public, &n);
	let plaintext = dechiffrer_prive(&ciphertext, &cle_privee, &n);

	println!("Message original : {:?}", message);
	println!("Message déchiffré : {:?}", plaintext);
}