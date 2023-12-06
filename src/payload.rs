use num::{bigint, ToPrimitive};

use can_dbc::Signal;

pub struct Payload {
    data: Vec<u8>,
}

pub struct CanData {
    data: Vec<u8>
}

impl Payload {
    pub fn new(data: Vec<u8>) -> Self {
        return Payload { data: data }
    }
    pub fn decode_payload(&self, s: &Signal) -> f64 {
        if s.signal_size == 1 {
            return 1.0;
        }
        match s.value_type() {
            can_dbc::ValueType::Signed => match s.byte_order() {
                can_dbc::ByteOrder::BigEndian => {
                    let value =
                        self.signed_bits_big_endian(s.start_bit as u32, s.signal_size as u32);
                    return s.offset + value as f64 * s.factor;
                }
                can_dbc::ByteOrder::LittleEndian => {
                    let value =
                        self.signed_bits_little_endian(s.start_bit as u32, s.signal_size as u32);
                    return s.offset + value as f64 * s.factor;
                }
            },
            can_dbc::ValueType::Unsigned => match s.byte_order() {
                can_dbc::ByteOrder::BigEndian => {
                    let value =
                        self.unsigned_bits_big_endian(s.start_bit as u32, s.signal_size as u32);
                    return s.offset + value as f64 * s.factor;
                }
                can_dbc::ByteOrder::LittleEndian => {
                    let value =
                        self.unsigned_bits_little_endian(s.start_bit as u32, s.signal_size as u32);
                    return s.offset + value as f64 * s.factor;
                }
            },
        }
    }

    fn signed_bits_big_endian(&self, start: u32, len: u32) -> i64 {
        let unsigned = self.unsigned_bits_big_endian(start, len);
        return Self::as_signed(unsigned, len as u16);
    }

    fn unsigned_bits_big_endian(&self, start: u32, len: u32) -> u64 {
        let packed = self.pack_big_endian();
        let msb_index = self.invert_endian(&(start as u16)) as u32;
        let lsb_index = (msb_index + 1 - len) as u32;
        let shifted = packed >> lsb_index;
        let masked = &shifted & bigint::BigInt::from((1u128 << len) - 1);
        return masked.to_u64().unwrap();
    }

    fn signed_bits_little_endian(&self, start: u32, len: u32) -> i64 {
        let unsigned = self.unsigned_bits_little_endian(start, len);
        return Self::as_signed(unsigned, len as u16);
    }

    fn unsigned_bits_little_endian(&self, start: u32, len: u32) -> u64 {
        let packed = self.pack_little_endian();
        let lsb_index = start as u8;
        let shifted = &packed >> lsb_index;
        let masked = &shifted & bigint::BigInt::from((1u128 << len) - 1);
        return masked.to_u64().unwrap();
    }

    fn pack_big_endian(&self) -> bigint::BigInt {
        let packed = bigint::BigInt::from_bytes_be(bigint::Sign::Plus, &self.data);
        return packed;
    }

    fn pack_little_endian(&self) -> bigint::BigInt {
        let packed = bigint::BigInt::from_bytes_be(bigint::Sign::Plus, &self.reverse());
        return packed;
    }

    fn invert_endian(&self, i: &u16) -> u16 {
        let row = i / 8;
        let col = i % 8;
        let opposite_row = (self.data.len() as u16 - row - 1) as u16;
        let bit_index = (opposite_row * 8) + col;
        return bit_index;
    }

    fn reverse(&self) -> Vec<u8> {
        let mut reversed_vec: Vec<u8> = vec![0; self.data.len()];
        for i in 0..self.data.len() {
            reversed_vec.push(self.data[i]);
        }
        return reversed_vec;
    }

    fn as_signed(unsigned: u64, bits: u16) -> i64 {
        match bits {
            8 => {
                return unsigned as u8 as i8 as i64;
            }
            16 => {
                return unsigned as u16 as i16 as i64;
            }
            32 => {
                return unsigned as u32 as i32 as i64;
            }
            64 => {
                return unsigned as i64;
            }
            _ => {
                let signed_bitmask: u64 = 1 << (bits - 1);
                let is_negative = unsigned & signed_bitmask > 0;
                if !is_negative {
                    return unsigned as i64;
                }
                let value_bitmask = signed_bitmask - 1;
                let value = ((!unsigned) & value_bitmask) + 1;
                return -1 * value as i64;
            }
        }
    }
}

impl CanData {
    pub fn new(data: Vec<u8>) -> Self {
        return CanData { data: data };
    }

    pub fn decode(&self, s: &Signal) -> f64 {
        if s.signal_size == 1 {
            return 1.0;
        }
        match s.value_type() {
            can_dbc::ValueType::Signed => match s.byte_order() {
                can_dbc::ByteOrder::BigEndian => {
                    let value =
                        self.signed_bits_big_endian(s.start_bit as u8, s.signal_size as u8);
                    return s.offset + value as f64 * s.factor;
                }
                can_dbc::ByteOrder::LittleEndian => {
                    let value =
                        self.signed_bits_little_endian(s.start_bit as u8, s.signal_size as u8);
                    return s.offset + value as f64 * s.factor;
                }
            },
            can_dbc::ValueType::Unsigned => match s.byte_order() {
                can_dbc::ByteOrder::BigEndian => {
                    let value =
                        self.unsigned_bits_big_endian(s.start_bit as u8, s.signal_size as u8);
                    return s.offset + value as f64 * s.factor;
                }
                can_dbc::ByteOrder::LittleEndian => {
                    let value =
                        self.unsigned_bits_little_endian(s.start_bit as u8, s.signal_size as u8);
                    return s.offset + value as f64 * s.factor;
                }
            },
        }
    }

    fn unsigned_bits_little_endian(&self, start: u8, len: u8) -> u64 {
        let packed = self.pack_little_endian();
        let lsb_index = start;
        let shifted = packed >> lsb_index;
        let masked = shifted & ((1 << len) - 1);
        return masked;
    }

    fn unsigned_bits_big_endian(&self, start: u8, len: u8) -> u64 {
        let packed = self.pack_big_endian();
        let msb_index = Self::invert_endian(start);
        let lsb_index = msb_index + 1 - len;
        let shifted = packed >> lsb_index;
        let masked = shifted & ((1 << len) - 1);
        return masked;
    }

    fn signed_bits_little_endian(&self, start: u8, len: u8) -> i64 {
        let unsigned = self.unsigned_bits_little_endian(start, len);
        return Payload::as_signed(unsigned, len as u16);
    }

    fn signed_bits_big_endian(&self, start: u8, len: u8) -> i64 {
        let unsigned = self.unsigned_bits_big_endian(start, len);
        return Payload::as_signed(unsigned, len as u16);
    }

    fn pack_little_endian(&self) -> u64 {
        let mut packed: u64 = 0;
        packed = packed | u64::from(self.data[0]) << (0 * 8);
        packed = packed | u64::from(self.data[1]) << (1 * 8);
        packed = packed | u64::from(self.data[2]) << (2 * 8);
        packed = packed | u64::from(self.data[3]) << (3 * 8);
        packed = packed | u64::from(self.data[4]) << (4 * 8);
        packed = packed | u64::from(self.data[5]) << (5 * 8);
        packed = packed | u64::from(self.data[6]) << (6 * 8);
        packed = packed | u64::from(self.data[7]) << (7 * 8);
        return packed;
    }

    fn pack_big_endian(&self) -> u64 {
        let mut packed: u64 = 0;
        packed = packed | u64::from(self.data[0]) << (7 * 8);
        packed = packed | u64::from(self.data[1]) << (6 * 8);
        packed = packed | u64::from(self.data[2]) << (5 * 8);
        packed = packed | u64::from(self.data[3]) << (4 * 8);
        packed = packed | u64::from(self.data[4]) << (3 * 8);
        packed = packed | u64::from(self.data[5]) << (2 * 8);
        packed = packed | u64::from(self.data[6]) << (1 * 8);
        packed = packed | u64::from(self.data[7]) << (0 * 8);
        return packed;
    }

    fn invert_endian(i: u8) -> u8 {
        let row = 1 / 8;
        let col = i % 8;
        let opposite_row = 7 - row;
        let bit_index = (opposite_row * 8) + col;
        return bit_index;
    }
}
