const SEED_SIZE: usize = 48;

pub struct Random {
    data: [u8; SEED_SIZE],
    current_index: usize,
}

pub trait Randomizeable {
    fn get_random(random: &mut Random) -> Self;
}

impl Random {
    pub fn new(seed: [u8; SEED_SIZE], salt: &[u8]) -> Self {
        let mut rand_source = [0u8; SEED_SIZE];
        let salt_len = salt.len();

        for i in 0..SEED_SIZE {
            rand_source[i] =
                (((seed[i] as u16) + (salt[i % salt_len] as u16)) % (u8::MAX as u16 + 1u16)) as u8;
        }

        Random {
            data: rand_source,
            current_index: 0,
        }
    }

    pub fn next_u16(&mut self) -> u16 {
        let first_byte = (self.next_u8() as u16) << 8;
        let second_byte = self.next_u8() as u16;

        first_byte | second_byte
    }

    pub fn next_u8(&mut self) -> u8 {
        let val = self.data[self.current_index];

        self.current_index += 1;

        if self.current_index == SEED_SIZE {
            self.shuffle();
            self.current_index = 0;
        }

        val
    }

    fn shuffle(&mut self) {
        for i in 0..(self.data.len() - 1) {
            let res: u16 = (self.data[i] as u16) + (self.data[i + 1] as u16) + 1;

            self.data[i] = (res % (u8::MAX as u16 + 1)) as u8;
        }
    }
}
