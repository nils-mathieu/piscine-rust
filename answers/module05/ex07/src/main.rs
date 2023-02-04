use std::io::{Read, Write};

use rand::{Rng, SeedableRng};
use rug::integer::Order;
use rug::Integer as UInt;

const SIZE: usize = 8;
const ORDER: Order = Order::Lsf;

fn main() {
    let mut args = std::env::args();
    args.next();
    let Some(cmd) = args.next() else {
            eprintln!("error: missing command");
            return;
        };

    match cmd.as_str() {
        "gen-keys" => {
            let Some(public_key) = args.next() else {
                eprintln!("error: missing public key");
                return;
            };

            let Some(private_key) = args.next() else {
                eprintln!("error: missing private key");
                return;
            };

            if args.next().is_some() {
                eprintln!("error: too many arguments");
                return;
            }

            let (e, d, m) = gen_keys();

            if let Err(err) = std::fs::write(&public_key, encode_key(&e, &m)) {
                eprintln!("error: {public_key}: {err}");
                return;
            }

            if let Err(err) = std::fs::write(&private_key, encode_key(&d, &m)) {
                eprintln!("error: {private_key}: {err}");
            }
        }
        "encrypt" => {
            let Some(public_key) = args.next() else {
                eprintln!("error: missing public key");
                return;
            };

            if args.next().is_some() {
                eprintln!("error: too many arguments");
                return;
            }

            encrypt_message(&public_key);
        }
        "decrypt" => {
            let Some(private_key) = args.next() else {
                eprintln!("error: missing private key");
                return;
            };

            if args.next().is_some() {
                eprintln!("error: too many arguments");
                return;
            }

            decrypt_message(&private_key);
        }
        _ => eprintln!("error: unknown command"),
    }
}

/// Encodes a key.
fn encode_key(exp: &UInt, m: &UInt) -> Vec<u8> {
    let mut result = Vec::new();
    writeln!(result, "{exp}").unwrap();
    writeln!(result, "{m}").unwrap();
    result
}

/// Decodes a key from a file.
fn decode_key(file: &[u8]) -> Option<(UInt, UInt)> {
    let mut splits = file.split(|&b| b == b'\n');
    let exp = splits.next()?;
    let m = splits.next()?;
    if !matches!(splits.next(), Some([]) | None) {
        return None;
    }

    let exp = std::str::from_utf8(exp).ok()?.parse::<UInt>().ok()?;
    let m = std::str::from_utf8(m).ok()?.parse::<UInt>().ok()?;

    Some((exp, m))
}

/// Generates a random prime number.
fn random_prime(rng: &mut impl Rng) -> UInt {
    UInt::from_digits(&std::array::from_fn::<u8, SIZE, _>(|_| rng.gen()), ORDER).next_prime()
}

fn random_e(rng: &mut impl Rng, phi: &UInt, m: &UInt) -> UInt {
    loop {
        let candidate = UInt::from_digits(
            &std::array::from_fn::<u8, { SIZE * 2 }, _>(|_| rng.gen()),
            ORDER,
        ) % phi;

        if candidate.clone().gcd(phi) == 1u32 && candidate.clone().gcd(m) == 1u32 {
            break candidate;
        }
    }
}

fn multiplicative_inverse(e: &UInt, phi: &UInt) -> UInt {
    let (_gcd, s, _t) = e.clone().extended_gcd(phi.clone(), UInt::new());
    (s % phi + phi) % phi
}

/// Generates a valid (E, D, M) tuple.
fn gen_keys() -> (UInt, UInt, UInt) {
    let mut rng = rand::rngs::StdRng::from_entropy();
    let p = random_prime(&mut rng);
    let q = random_prime(&mut rng);

    let m = p.clone() * &q;
    let phi = (p - 1) * (q - 1);

    let e = random_e(&mut rng, &phi, &m);
    let d = multiplicative_inverse(&e, &phi);

    (e, d, m)
}

fn block_size(m: &UInt) -> usize {
    let mut b = 0usize;
    let mut pow = UInt::from(1u32);
    loop {
        pow *= 255u32;
        if pow > *m {
            break b;
        }
        b += 1;
    }
}

fn decode_keyfile(keyfile: &str) -> Option<(UInt, UInt)> {
    let key = match std::fs::read(keyfile) {
        Ok(k) => k,
        Err(err) => {
            eprintln!("error: {keyfile}: {err}");
            return None;
        }
    };

    let Some((e, m)) = decode_key(&key) else {
        eprintln!("error: {keyfile}: invalid key format");
        return None;
    };

    Some((e, m))
}

fn encrypt_message(keyfile: &str) {
    let Some((e, m)) = decode_keyfile(keyfile) else { return; };
    let c = block_size(&m);
    let ec = c + 1;

    let mut message = Vec::new();
    if let Err(err) = std::io::stdin().read_to_end(&mut message) {
        eprintln!("error: stdin: {err}");
        return;
    }

    let mut result = Vec::<u8>::new();
    result.resize((message.len() / c + 1) * ec, 0);
    for (i, chunk) in message.chunks(c).enumerate() {
        let mut chunk = UInt::from_digits(chunk, ORDER);
        chunk.pow_mod_mut(&e, &m).unwrap();
        chunk.write_digits(&mut result[i * ec..], ORDER);
    }

    if let Err(err) = std::io::stdout().write_all(&result) {
        eprintln!("error: stdout: {err}");
    }
}

fn decrypt_message(keyfile: &str) {
    let Some((e, m)) = decode_keyfile(keyfile) else { return; };
    let c = block_size(&m);
    let ec = c + 1;

    let mut message = Vec::new();
    if let Err(err) = std::io::stdin().read_to_end(&mut message) {
        eprintln!("error: stdin: {err}");
        return;
    }

    let mut result = Vec::<u8>::new();
    result.resize((message.len() / ec + 1) * c, 0);
    for (i, chunk) in message.chunks(ec).enumerate() {
        let mut chunk = UInt::from_digits(chunk, ORDER);
        chunk.pow_mod_mut(&e, &m).unwrap();
        if chunk.significant_digits::<u8>() <= c {
            chunk.write_digits(&mut result[i * c..], ORDER);
        }
    }

    if let Err(err) = std::io::stdout().write_all(&result) {
        eprintln!("error: stdout: {err}");
    }
}
