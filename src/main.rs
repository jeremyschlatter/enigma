const ALPHABET_SIZE: usize = 26;
type Permutation = [usize; ALPHABET_SIZE];

const PERMUTATIONS: [Permutation; 2] = [
    [
        0, 21, 4, 7, 15, 18, 12, 14, 16, 8, 3, 19, 24, 23, 2, 11, 13, 5, 22, 20, 6, 25, 10, 17, 9,
        1,
    ],
    [
        5, 22, 8, 24, 14, 16, 7, 11, 10, 18, 6, 15, 9, 25, 0, 2, 13, 3, 23, 21, 12, 20, 4, 17, 19,
        1,
    ],
];

#[derive(Debug)]
struct Enigma {
    rotor: Rotor,
    reflector: Reflector,
}

fn u2c(u: usize) -> char {
    (u + ('a' as u8 as usize)) as u8 as char
}

fn c2u(c: char) -> usize {
    c as usize - 'a' as usize
}

impl Enigma {
    fn default() -> Enigma {
        Enigma {
            rotor: Rotor::from(PERMUTATIONS[0]),
            reflector: Reflector::from(PERMUTATIONS[1]),
        }
    }
    fn cipher(&mut self, s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        for c in s.chars() {
            result.push(self.cipher_one(c))
        }
        result
    }

    fn cipher_one(&mut self, c: char) -> char {
        let mut u = c2u(c);
        u = self.rotor.forward(u);
        u = self.reflector.reflect(u);
        u = self.rotor.backward(u);
        self.rotor.offset = (self.rotor.offset + 1) % ALPHABET_SIZE;
        u2c(u)
    }
}

#[derive(Debug)]
struct Reflector {
    wiring: Permutation,
}

impl Reflector {
    fn from(p: Permutation) -> Reflector {
        let mut r = p.clone();
        for (a, b) in p.iter().zip(p.iter().rev()) {
            r[*a] = *b;
            r[*b] = *a;
        }

        Reflector { wiring: r }
    }
    fn reflect(&self, i: usize) -> usize {
        self.wiring[i]
    }
}

#[derive(Debug)]
struct Rotor {
    wiring: Permutation,
    wiring_backward: Permutation,
    offset: usize,
}

impl Rotor {
    fn from(p: Permutation) -> Rotor {
        let mut r = Rotor {
            wiring: p,
            wiring_backward: p,
            offset: 0,
        };
        for i in 0..ALPHABET_SIZE {
            r.wiring[i] = (ALPHABET_SIZE + p[i] - i) % ALPHABET_SIZE;
            r.wiring_backward[p[i]] = (ALPHABET_SIZE + i - p[i]) % ALPHABET_SIZE;
        }
        r
    }

    fn forward(&self, i: usize) -> usize {
        (i + self.wiring[(i + self.offset) % ALPHABET_SIZE]) % ALPHABET_SIZE
    }
    fn backward(&self, i: usize) -> usize {
        (i + self.wiring_backward[(i + self.offset) % ALPHABET_SIZE]) % ALPHABET_SIZE
    }
}

fn main() {
    let mut enigma = Enigma::default();

    println!("{:?}", enigma.cipher("svoolcpbov"));
    println!("{:?}", enigma.cipher("helloxkyle"));
}

#[test]
fn it_is_symmetric() {
    let plaintext = "helloxkyle";
    let ciphertext = Enigma::default().cipher(plaintext);

    assert_eq!(Enigma::default().cipher(&ciphertext), plaintext);
}

#[test]
fn it_steps() {
    let mut enigma = Enigma::default();
    assert_ne!(enigma.cipher("a"), enigma.cipher("a"));
}
