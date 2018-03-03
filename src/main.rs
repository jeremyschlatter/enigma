const ALPHABET_SIZE: usize = 26;
type Permutation = [u8; ALPHABET_SIZE];

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

fn u2c(u: u8) -> char {
    (u + ('a' as u8)) as char
}

fn c2u(c: char) -> u8 {
    c as u8 - 'a' as u8
}

impl Enigma {
    fn default() -> Enigma {
        Enigma {
            rotor: Rotor::from(PERMUTATIONS[0]),
            reflector: Reflector::from(PERMUTATIONS[1]),
        }
    }
    fn cipher(&self, s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        for c in s.chars() {
            result.push(self.cipher_one(c))
        }
        result
    }

    fn cipher_one(&self, c: char) -> char {
        let mut u = c2u(c);
        u = self.rotor.forward(u);
        u = self.reflector.reflect(u);
        u = self.rotor.backward(u);
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
            r[*a as usize] = *b;
            r[*b as usize] = *a;
        }
        println!("{:?}", r);
        Reflector { wiring: r }
    }
    fn reflect(&self, i: u8) -> u8 {
        self.wiring[i as usize]
    }
}

#[derive(Debug)]
struct Rotor {
    forward_: Permutation,
    backward_: Permutation,
    offset: u8,
}

impl Rotor {
    fn from(p: Permutation) -> Rotor {
        let mut r = Rotor {
            forward_: p,
            backward_: p,
            offset: 0,
        };
        for i in 0..ALPHABET_SIZE {
            r.backward_[r.forward_[i] as usize] = i as u8;
        }
        r
    }
    fn forward(&self, i: u8) -> u8 {
        self.forward_[i as usize]
    }
    fn backward(&self, i: u8) -> u8 {
        self.backward_[i as usize]
    }
}

fn main() {
    let enigma = Enigma::default();

    println!("{:?}", enigma.cipher("svoolcpbov"));
    println!("{:?}", enigma.cipher("helloxkyle"));
}

#[test]
fn it_is_symmetric() {
    let plaintext = "helloxkyle";
    let enigma = Enigma::default();
    assert_eq!(enigma.cipher(&enigma.cipher(plaintext)), plaintext);
}
