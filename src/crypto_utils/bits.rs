use num::{FromPrimitive, Num, ToPrimitive};
use std::mem::size_of;
use std::ops::{BitAnd, BitOr, Shl, Shr, ShrAssign, Sub};

pub fn split_into_bytes<T>(n: T) -> Vec<u8>
where
    T: Num + Ord + Copy + ShrAssign<u8> + BitAnd<T, Output = T> + FromPrimitive + ToPrimitive,
{
    let mut bytes = Vec::new();
    let mut num = n;

    // TODO: seems to be unbreakable, but need to check it more deeply
    while num > T::zero() {
        let byte_value = &(num & FromPrimitive::from_u8(0xFF).unwrap());
        bytes.push(ToPrimitive::to_u8(byte_value).unwrap());
        num >>= 8;
    }

    bytes.reverse();
    bytes
}

pub enum Direction {
    Left,
    Right,
}

pub fn rotate<T>(x: T, n: u8, dir: Direction) -> T
where
    T: Num
        + FromPrimitive
        + Shr<T, Output = T>
        + Shl<T, Output = T>
        + Sub<T, Output = T>
        + BitOr<T, Output = T>
        + Copy,
{
    let bits: T = FromPrimitive::from_usize(size_of::<T>() * 8).unwrap();
    let n = FromPrimitive::from_u8(n).unwrap();
    match dir {
        Direction::Left => (x << n) | (x >> (bits - n)),
        Direction::Right => (x >> n) | (x << (bits - n)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_bytes() {
        let x = 0x12345678u32;
        let bytes = split_into_bytes(x);

        assert_eq!(bytes, vec![0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn test_rotate() {
        let x = 0x12345678u32;
        let y = 0b11110000u8;

        let left_x = rotate(x, 4, Direction::Left);
        let right_x = rotate(x, 4, Direction::Right);

        let left_y = rotate(y, 2, Direction::Left);
        let right_y = rotate(y, 2, Direction::Right);

        assert_eq!(left_x, 0x23456781);
        assert_eq!(right_x, 0x81234567);

        assert_eq!(left_y, 0b11000011);
        assert_eq!(right_y, 0b00111100);
    }
}
