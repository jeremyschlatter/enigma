const ALPHABET_SIZE: usize = 26;
type Permutation = [usize; ALPHABET_SIZE];

const PERMUTATIONS: [Permutation; 4] = [
    [
        0, 21, 4, 7, 15, 18, 12, 14, 16, 8, 3, 19, 24, 23, 2, 11, 13, 5, 22, 20, 6, 25, 10, 17, 9,
        1,
    ],
    [
        5, 22, 8, 24, 14, 16, 7, 11, 10, 18, 6, 15, 9, 25, 0, 2, 13, 3, 23, 21, 12, 20, 4, 17, 19,
        1,
    ],
    [
        25, 4, 6, 20, 13, 21, 14, 12, 22, 11, 0, 17, 9, 16, 10, 15, 5, 19, 8, 1, 7, 3, 2, 24, 23,
        18,
    ],
    [
        2, 17, 9, 1, 21, 12, 15, 11, 20, 3, 24, 14, 4, 10, 16, 22, 23, 5, 19, 7, 25, 6, 18, 13, 0,
        8,
    ],
];

#[derive(Debug)]
struct Enigma {
    rotors: [Rotor; 3],
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
            rotors: [
                Rotor::from(PERMUTATIONS[0], 3),
                Rotor::from(PERMUTATIONS[1], 5),
                Rotor::from(PERMUTATIONS[2], 10),
            ],
            reflector: Reflector::from(PERMUTATIONS[3]),
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
        for rotor in self.rotors.iter() {
            u = rotor.forward(u);
        }
        u = self.reflector.reflect(u);
        for rotor in self.rotors.iter().rev() {
            u = rotor.backward(u);
        }
        for i in 0..self.rotors.len() {
            if i == 0 || self.rotors[i - 1].notch == self.rotors[i - 1].offset
                // double-stepping:
                || (i < self.rotors.len() && self.rotors[i].notch == self.rotors[i].offset)
            {
                self.rotors[i].offset = (self.rotors[i].offset + 1) % ALPHABET_SIZE;
            }
        }
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
    notch: usize,
}

impl Rotor {
    fn from(p: Permutation, notch: usize) -> Rotor {
        let mut r = Rotor {
            wiring: p,
            wiring_backward: p,
            offset: 0,
            notch: notch,
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

#[test]
fn it_steps_all_rotors() {
    let mut enigma = Enigma::default();
    for _ in 0..(ALPHABET_SIZE.pow(2) + ALPHABET_SIZE + 4) {
        enigma.cipher("a");
    }
    assert_eq!(
        (1, 3, 4),
        (
            enigma.rotors[2].offset,
            enigma.rotors[1].offset,
            enigma.rotors[0].offset
        )
    );
}
