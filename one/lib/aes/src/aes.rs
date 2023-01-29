use std::array::TryFromSliceError;

use crate::galois::GaloisField;

/// The AES SBOX
const S_BOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

/// The supported key sizes
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeySize {
    /// A 128 bit key
    AES_128,
    /// A 192 bit key
    AES_192,
    /// A 256 bit key
    AES_256,
}

/// A trait that describes all of the possible key operations
pub trait AesKeyOps<const N: usize, const N_ROUND_KEYS: usize> {
    /// Performs an SBOX lookup with a given u32
    fn sub_bytes(a: u32) -> u32;
    /// Rotates an integer 8 bits to the left
    fn rot_bytes(a: u32) -> u32;
    /// Derives round keys from the original
    fn derive_round_keys(&self) -> [AesRoundKey; N_ROUND_KEYS]
    where
        AesRoundKey: Sized;
}

/// A struct that represents an AES round key
/// This must always be 128 bits
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct AesRoundKey {
    data: [u32; 4],
}

impl TryFrom<&[u32]> for AesRoundKey {
    type Error = TryFromSliceError;

    fn try_from(value: &[u32]) -> Result<Self, Self::Error> {
        let data: [u32; 4] = value.try_into()?;
        Ok(Self { data })
    }
}

impl AesKeyOps<4, 0> for AesRoundKey {
    /// This is not applicable for round keys
    fn sub_bytes(_a: u32) -> u32 {
        unimplemented!();
    }

    /// This is not applicable for round keys
    fn rot_bytes(_a: u32) -> u32 {
        unimplemented!();
    }

    /// This is not applicable for round keys
    fn derive_round_keys(&self) -> [Self; 0]
    where
        Self: Sized,
    {
        unimplemented!();
    }
}

/// Represents an AES key
/// This can be 128, 192, or 256 bits
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AesKey<const N: usize> {
    /// The key's size
    pub size: KeySize,
    /// The raw key data
    data: [u32; N],
}

impl AesKeyOps<4, 11> for AesKey<4> {
    fn rot_bytes(a: u32) -> u32 {
        a.rotate_left(8)
    }

    fn sub_bytes(a: u32) -> u32 {
        let mut a: [u8; 4] = a.to_be_bytes();
        for x in 0..4 {
            a[x] = S_BOX[((a[x] >> 4) * 16 + (a[x] << 4 >> 4)) as usize]
        }

        u32::from_be_bytes(a)
    }

    fn derive_round_keys(&self) -> [AesRoundKey; 11]
    where
        AesRoundKey: Sized,
    {
        const R_CON: [u32; 0x0A] = [
            0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 0x20000000, 0x40000000,
            0x80000000, 0x1B000000, 0x36000000,
        ];

        const NK: usize = 0x04;

        let mut words = [0u32; 44];

        // The first 6 words are the original key
        for (x, word) in words.iter_mut().enumerate().take(NK) {
            *word = self.data[x];
        }

        // Derives the rest of the words
        for x in NK..words.len() {
            if x % NK == 0 {
                words[x] = Self::sub_bytes(Self::rot_bytes(words[x - 1]))
                    ^ R_CON[x / NK - 1]
                    ^ words[x - NK];
                continue;
            }

            words[x] = words[x - 1] ^ words[x - NK];
        }

        let mut round_keys: [AesRoundKey; 11] = [AesRoundKey::default(); 11];
        for x in (0..words.len()).step_by(4) {
            round_keys[x / 4] = words[x..x + 4].try_into().unwrap();
        }

        round_keys
    }
}

impl AesKeyOps<6, 13> for AesKey<6> {
    fn rot_bytes(a: u32) -> u32 {
        a.rotate_left(8)
    }

    fn sub_bytes(a: u32) -> u32 {
        let mut a: [u8; 4] = a.to_be_bytes();
        for x in 0..4 {
            a[x] = S_BOX[((a[x] >> 4) * 16 + (a[x] << 4 >> 4)) as usize]
        }

        u32::from_be_bytes(a)
    }

    fn derive_round_keys(&self) -> [AesRoundKey; 13]
    where
        AesRoundKey: Sized,
    {
        const R_CON: [u32; 0x0A] = [
            0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 0x20000000, 0x40000000,
            0x80000000, 0x1B000000, 0x36000000,
        ];

        const NK: usize = 0x06;

        let mut words = [0u32; 13 * 4];

        // The first 6 words are the original key
        for (x, word) in words.iter_mut().enumerate().take(NK) {
            *word = self.data[x];
        }

        // Derives the rest of the words
        for x in NK..words.len() {
            if x % NK == 0 {
                words[x] = Self::sub_bytes(Self::rot_bytes(words[x - 1]))
                    ^ R_CON[x / NK - 1]
                    ^ words[x - NK];
                continue;
            }

            words[x] = words[x - 1] ^ words[x - NK];
        }

        let mut round_keys: [AesRoundKey; 13] = [AesRoundKey::default(); 13];
        for x in (0..words.len()).step_by(4) {
            round_keys[x / 4] = words[x..x + 4].try_into().unwrap();
        }

        round_keys
    }
}

impl AesKeyOps<8, 15> for AesKey<8> {
    fn rot_bytes(a: u32) -> u32 {
        a.rotate_left(8)
    }

    fn sub_bytes(a: u32) -> u32 {
        let mut a: [u8; 4] = a.to_be_bytes();
        for x in 0..4 {
            a[x] = S_BOX[((a[x] >> 4) * 16 + (a[x] << 4 >> 4)) as usize]
        }

        u32::from_be_bytes(a)
    }

    fn derive_round_keys(&self) -> [AesRoundKey; 15]
    where
        AesRoundKey: Sized,
    {
        const R_CON: [u32; 0x0A] = [
            0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000, 0x20000000, 0x40000000,
            0x80000000, 0x1B000000, 0x36000000,
        ];

        const NK: usize = 0x08;

        let mut words = [0u32; 60];

        // The first 6 words are the original key
        for (x, word) in words.iter_mut().enumerate().take(NK) {
            *word = self.data[x];
        }

        // Derives the rest of the words
        for x in NK..words.len() {
            if x % NK == 0 {
                words[x] = Self::sub_bytes(Self::rot_bytes(words[x - 1]))
                    ^ R_CON[x / NK - 1]
                    ^ words[x - NK];
                continue;
            } else if (x - (NK / 2)) % NK == 0 {
                words[x] = Self::sub_bytes(words[x - 1]) ^ words[x - NK];
                continue;
            }

            words[x] = words[x - 1] ^ words[x - NK];
        }

        let mut round_keys: [AesRoundKey; 15] = [AesRoundKey::default(); 15];
        for x in (0..words.len()).step_by(4) {
            round_keys[x / 4] = words[x..x + 4].try_into().unwrap();
        }

        round_keys
    }
}

impl Default for AesKey<4> {
    fn default() -> Self {
        Self {
            size: KeySize::AES_128,
            data: [0u32; 4],
        }
    }
}

impl Default for AesKey<6> {
    fn default() -> Self {
        Self {
            size: KeySize::AES_192,
            data: [0u32; 6],
        }
    }
}

impl Default for AesKey<8> {
    fn default() -> Self {
        Self {
            size: KeySize::AES_256,
            data: [0u32; 8],
        }
    }
}

/// The AES internal state, this represents a 
/// 128 bit block of data (can be both) plain
/// and cipher text
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct AesState(pub [u32; 4]);

/// Implements the operations that can be applied to the state
pub trait AesStateOps<const N: usize> {
    /// Gets a given row from the state
    fn row(&self, index: usize) -> [u8; N];
    /// Sets a given row in the state
    fn set_row(&mut self, index: usize, row: [u8; N]);
    /// Performs a lookup on the AES SBOX with the given byte
    fn sub_byte(a: u8) -> u8;
    /// Performs an SBOX lookup on the entire state
    fn sub_bytes(&mut self);
    /// Shifts each row of the state
    fn shift_rows(&mut self);
    /// Mixes a given column
    fn mix_column(a: u32) -> u32;
    /// Mixes each column of the state
    fn mix_columns(&mut self);
    /// XORs a given round key to the state
    fn add_round_key(&mut self, key: &AesRoundKey);
}

impl AesStateOps<4> for AesState {
    fn row(&self, index: usize) -> [u8; 4] {
        let mut row = [0u8; 4];
        for (x, item) in self.0.iter().enumerate() {
            row[x] = item.to_be_bytes()[index];
        }
        row
    }

    fn set_row(&mut self, index: usize, row: [u8; 4]) {
        for (x, item) in self.0.iter_mut().enumerate() {
            let mut y = item.to_be_bytes();
            y[index] = row[x];
            *item = u32::from_be_bytes(y);
        }
    }

    fn sub_byte(a: u8) -> u8 {
        S_BOX[((a >> 4) * 16 + (a << 4 >> 4)) as usize]
    }

    fn sub_bytes(&mut self) {
        for x in 0..4 {
            let mut row = self.0[x].to_be_bytes();
            for item in &mut row {
                *item = Self::sub_byte(*item);
            }
            self.0[x] = u32::from_be_bytes(row);
        }
    }

    fn shift_rows(&mut self) {
        for x in 0..4 {
            let mut row = self.row(x);
            row.rotate_left(x);
            self.set_row(x, row);
        }
    }

    fn mix_column(a: u32) -> u32 {
        const MATRIX: [[u8; 4]; 4] = [
            [0x02, 0x03, 0x01, 0x01],
            [0x01, 0x02, 0x03, 0x01],
            [0x01, 0x01, 0x02, 0x03],
            [0x03, 0x01, 0x01, 0x02],
        ];

        let v = a.to_be_bytes();
        let mut tmp: [u8; 4] = v;
        for x in 0..4 {
            tmp[x] = ((GaloisField::<u8>(v[0]) * MATRIX[x][0])
                ^ (GaloisField::<u8>(v[1]) * MATRIX[x][1])
                ^ (GaloisField::<u8>(v[2]) * MATRIX[x][2])
                ^ (GaloisField::<u8>(v[3]) * MATRIX[x][3]))
                .0;
        }

        u32::from_be_bytes(tmp)
    }

    fn mix_columns(&mut self) {
        for x in 0..4 {
            self.0[x] = Self::mix_column(self.0[x]);
        }
    }

    fn add_round_key(&mut self, key: &AesRoundKey) {
        for x in 0..4 {
            self.0[x] ^= key.data[x];
        }
    }
}

/// The AES Modes of operation
/// Currently only ECB is supported 
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Default)]
pub enum AesMode {
    /// ECB (Electroni Code Book)
    #[default]
    AES_ECB,
    // AES_CBC,
    // AES_CFB,
    // AES_OFB,
    // AES_CTR,
}

/// Handles the AES state
/// This is responsible for encrypting or 
/// decrypting given data
#[derive(Copy, Clone, Debug, Default)]
pub struct Aes {
    /// The AES mode of operation
    pub mode: AesMode,
    /// The internal state
    state: AesState,
}

impl Aes {
    fn round(&mut self, round_key: &AesRoundKey) {
        self.state.sub_bytes();
        self.state.shift_rows();
        self.state.mix_columns();
        self.state.add_round_key(round_key);
    }

    fn final_round(&mut self, round_key: &AesRoundKey) {
        self.state.sub_bytes();
        self.state.shift_rows();
        self.state.add_round_key(round_key);
    }

    /// Loads raw data into the AES 128 bit state
    pub fn load_state(&mut self, data: AesState) {
        self.state = data;
    }

    /// Encrypts a block with a given key
    pub fn encrypt_block_128(&mut self, key: &AesKey<4>) {
        const N_ROUNDS: usize = 0x0A;

        // Derives the round keys
        let round_keys = key.derive_round_keys();

        // Adds the first round key
        self.state.add_round_key(&round_keys[0]);

        // Executes rounds
        for x in 0..N_ROUNDS - 1 {
            self.round(&round_keys[x + 1]);
        }
        self.final_round(&round_keys[N_ROUNDS]);
    }

    /// Encrypts a block with a given key
    pub fn encrypt_block_192(&mut self, key: &AesKey<6>) {
        const N_ROUNDS: usize = 0x0C;

        // Derives the round keys
        let round_keys = key.derive_round_keys();

        // Adds the first round key
        self.state.add_round_key(&round_keys[0]);

        // Executes rounds
        for x in 0..N_ROUNDS - 1 {
            self.round(&round_keys[x + 1]);
        }
        self.final_round(&round_keys[N_ROUNDS]);
    }

    /// Encrypts a block with a given key
    pub fn encrypt_block_256(&mut self, key: &AesKey<8>) {
        const N_ROUNDS: usize = 0x0E;

        // Derives the round keys
        let round_keys = key.derive_round_keys();

        // Adds the first round key
        self.state.add_round_key(&round_keys[0]);

        // Executes rounds
        for x in 0..N_ROUNDS - 1 {
            self.round(&round_keys[x + 1]);
        }
        self.final_round(&round_keys[N_ROUNDS]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_128_rot_bytes() {
        assert_eq!(AesKey::<4>::rot_bytes(0x01020304), 0x02030401);
    }

    #[test]
    fn key_192_rot_bytes() {
        assert_eq!(AesKey::<6>::rot_bytes(0x01020304), 0x02030401);
    }

    #[test]
    fn key_256_rot_bytes() {
        assert_eq!(AesKey::<8>::rot_bytes(0x01020304), 0x02030401);
    }

    #[test]
    fn key_128_sub_bytes() {
        assert_eq!(AesKey::<4>::sub_bytes(0x01200340), 0x7CB77B09);
    }

    #[test]
    fn key_192_sub_bytes() {
        assert_eq!(AesKey::<6>::sub_bytes(0x01200340), 0x7CB77B09);
    }

    #[test]
    fn key_256_sub_bytes() {
        assert_eq!(AesKey::<8>::sub_bytes(0x01200340), 0x7CB77B09);
    }

    #[test]
    fn key_128_derive_round_keys() {
        let key = AesKey {
            size: KeySize::AES_128,
            data: [0x0F1571C9, 0x47D9E859, 0x0CB7ADD6, 0xAF7F6798],
        };

        let result = AesRoundKey {
            data: [0xB48EF352, 0xBA98134E, 0x7F4D5920, 0x86261876],
        };

        let round_keys = key.derive_round_keys();

        assert_eq!(*round_keys.last().unwrap(), result);
    }

    #[test]
    fn key_192_derive_round_keys() {
        let key = AesKey {
            size: KeySize::AES_192,
            data: [
                0x8E73B0F7, 0xDA0E6452, 0xc810F32B, 0x809079E5, 0x62F8EAD2, 0x522C6b7B,
            ],
        };

        let result = AesRoundKey {
            data: [0xE98BA06F, 0x448C773C, 0x8ECC7204, 0x1002202],
        };

        let round_keys = key.derive_round_keys();

        assert_eq!(*round_keys.last().unwrap(), result);
    }

    #[test]
    fn key_256_derive_round_keys() {
        let key = AesKey {
            size: KeySize::AES_256,
            data: [
                0x603deb10, 0x15ca71be, 0x2b73aef0, 0x857d7781, 0x1f352c07, 0x3b6108d7, 0x2d9810a3,
                0x0914dff4,
            ],
        };

        let result = AesRoundKey {
            data: [0xfe4890d1, 0xe6188d0b, 0x46df344, 0x706c631e],
        };

        let round_keys = key.derive_round_keys();

        assert_eq!(*round_keys.last().unwrap(), result);
    }

    #[test]
    fn state_128_row() {
        let state = AesState([0x01020304, 0x05060708, 0x090A0B0C, 0x0D0E0F00]);

        assert_eq!(state.row(1), [0x02, 0x06, 0x0A, 0x0E]);
        assert_eq!(state.row(3), [0x04, 0x08, 0x0C, 0x00]);
    }

    #[test]
    fn state_128_set_row() {
        let mut state = AesState([0x01020304, 0x05060708, 0x090A0B0C, 0x0D0E0F00]);

        state.set_row(1, [0x10, 0x20, 0x30, 0x40]);
        state.set_row(3, [0x00, 0x00, 0x00, 0x00]);

        assert_eq!(state.row(1), [0x10, 0x20, 0x30, 0x40]);
        assert_eq!(state.row(3), [0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn state_128_sub_byte() {
        assert_eq!(AesState::sub_byte(0xFF), 0x16);
        assert_eq!(AesState::sub_byte(0x00), 0x63);
        assert_eq!(AesState::sub_byte(0xE2), 0x98);
        assert_eq!(AesState::sub_byte(0x7B), 0x21);
    }

    #[test]
    fn state_128_sub_bytes() {
        let mut state = AesState([0x01020304, 0x05060708, 0x090A0B0C, 0x0D0E0F00]);
        let result = AesState([0x7c777bf2, 0x6b6fc530, 0x01672bfe, 0xd7ab7663]);

        state.sub_bytes();
        assert_eq!(state, result);
    }

    #[test]
    fn state_128_shift_rows() {
        let mut state = AesState([0xD42711AE, 0xE0BF98F1, 0xB8B45DE5, 0x1E415230]);
        let result = AesState([0xD4BF5D30, 0xE0B452AE, 0xB84111F1, 0x1E2798E5]);

        state.shift_rows();
        assert_eq!(state, result);
    }

    #[test]
    fn state_128_mix_column() {
        assert_eq!(AesState::mix_column(0xDB135345), 0x8E4DA1BC);
        assert_eq!(AesState::mix_column(0xF20A225C), 0x9FDC589D);
        assert_eq!(AesState::mix_column(0x2D26314C), 0x4D7EBDF8);
    }

    #[test]
    fn state_128_mix_columns() {
        let mut state = AesState([0xDB135345, 0xF20A225C, 0x2D26314C, 0xC6C6C6C6]);
        let result = AesState([0x8E4DA1BC, 0x9FDC589D, 0x4D7EBDF8, 0xC6C6C6C6]);

        state.mix_columns();
        assert_eq!(state, result);
    }

    #[test]
    fn state_128_add_round_key() {
        let mut state = AesState([0xB9945775, 0xE48E1651, 0x47209A3F, 0xC5D6F53B]);

        let round_key = AesRoundKey {
            data: [0xDC9B9738, 0x9049FE81, 0x37DF7215, 0xB0E93FA7],
        };

        let result = AesState([0x650FC04D, 0x74C7E8D0, 0x70FFE82A, 0x753FCA9C]);

        state.add_round_key(&round_key);
        assert_eq!(state, result);
    }

    #[test]
    fn aes_encrypt_block_128() {
        let key = AesKey::<4> {
            size: KeySize::AES_128,
            data: [0x2b7E1516, 0x28AED2A6, 0xABF71588, 0x09CF4F3C],
        };

        let result = AesState([0x3925841D, 0x2DC09FB, 0xDC118597, 0x196A0B32]);

        let mut aes = Aes::default();
        aes.load_state(AesState([0x3243F6A8, 0x885A308D, 0x313198A2, 0xE0370734]));
        aes.encrypt_block_128(&key);

        assert_eq!(aes.state, result);
    }

    #[test]
    fn aes_encrypt_block_192() {
        let key = AesKey::<6> {
            size: KeySize::AES_192,
            data: [
                0x00010203, 0x04050607, 0x08090A0B, 0x0C0D0E0F, 0x10111213, 0x14151617,
            ],
        };

        let result = AesState([0xDDA97CA4, 0x864CDFE0, 0x6EAF70A0, 0xEC0D7191]);

        let mut aes = Aes::default();
        aes.load_state(AesState([0x00112233, 0x44556677, 0x8899AABB, 0xCCDDEEFF]));
        aes.encrypt_block_192(&key);

        assert_eq!(aes.state, result);
    }

    #[test]
    fn aes_encrypt_block_256() {
        let key = AesKey::<8> {
            size: KeySize::AES_256,
            data: [
                0x00010203, 0x04050607, 0x08090A0B, 0x0C0D0E0F, 0x10111213, 0x14151617, 0x18191A1B,
                0x1C1D1E1F,
            ],
        };

        let result = AesState([0x8EA2b7CA, 0x516745BF, 0xEAFC4990, 0x4B496089]);

        let mut aes = Aes::default();
        aes.load_state(AesState([0x00112233, 0x44556677, 0x8899AABB, 0xCCDDEEFF]));
        aes.encrypt_block_256(&key);

        assert_eq!(aes.state, result);
    }
}
