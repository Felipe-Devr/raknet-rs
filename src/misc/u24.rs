use std::ops::Add;



#[allow(non_camel_case_types)]
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct u24([u8; 3]);

impl Add for u24 {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self::from_u32(self.to_u32() + other.to_u32())
	}
}

impl u24 {
	pub fn to_u32(self) -> u32 {
		let u24([a,b,c]) = self;
		u32::from_le_bytes([a, b, c, 0])
	}

	pub fn from_u32(n: u32) -> Self {
		let [a, b, c, d] = n.to_le_bytes();
		debug_assert!(d == 0);
		Self([a, b, c])
	}
}