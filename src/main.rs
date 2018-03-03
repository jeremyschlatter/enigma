mod rotors;

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
    fn from(rotor_set: [[u8; 26]; 2]) -> Enigma {
        Enigma {
            rotor: Rotor::from(rotor_set[0]),
            reflector: Reflector {
                wiring: rotor_set[1],
            },
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
    // TODO: Do the folding in a constructor instead of in Python.
    wiring: [u8; 26],
}

impl Reflector {
    fn reflect(&self, i: u8) -> u8 {
        self.wiring[i as usize]
    }
}

#[derive(Debug)]
struct Rotor {
    forward_: [u8; 26],
    backward_: [u8; 26],
    offset: u8,
}

impl Rotor {
    fn from(arr: [u8; 26]) -> Rotor {
        let mut r = Rotor {
            forward_: arr,
            backward_: arr,
            offset: 0,
        };
        for i in 0..26 {
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
    let enigma = Enigma::from(rotors::rotors());

    println!("{:?}", enigma.cipher("svoolcpbov"));
    println!("{:?}", enigma.cipher("helloxkyle"));
}

#[test]
fn it_is_symmetric() {
    let plaintext = "helloxkyle";
    let enigma = Enigma::from(rotors::rotors());
    assert_eq!(enigma.cipher(&enigma.cipher(plaintext)), plaintext);
}
