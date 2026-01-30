fn main() {
    println!("Hello, world!");
}

/*
signed and unsigned integers: i and u.

scalar types:
- intergers: number without a fractional component.
length 8 bit - signed i8
length 16 bit - signed i16
length 32 bit - signed i32
length 64 bit - signed i64
length 128 bit - signed i128
length 256 bit - signed i256

architecture dependent: signed is isize and Unsigned is usize.

length 8 bit - unsigned u8
length 16 bit - unsigned u16
length 32 bit - unsigned u32
length 64 bit - unsigned u64
length 128 bit - unsigned u128
length 256 bit - unsigned u256

each signed variant can store numbers from -(2^(n-1)) to (2^(n-1) - 1) inclusive. where n is the number of bits the varient uses, so an i8 can store numbers from -(2^7) to (2^7 - 1) inclusive. which is -128 to 127.

*/
