use std::collections::VecDeque;
use std::fmt;

pub struct Packet
{
	pub payload: VecDeque<u8>,
}

pub struct Header
{
	pub data: Vec<u8>,
}

impl Packet
{
	pub fn new() -> Self
	{
		let payload = VecDeque::with_capacity(32);
		Packet{payload}
	}

	pub fn len(&self) -> usize
	{
		self.payload.len()
	}

	pub fn push_header(&mut self, header: &Header)
	{
		self.payload.reserve(header.data.len());
		for b in header.data.iter().rev() {
			self.payload.push_front(*b);
		}
	}

	pub fn push_back_bytes(&mut self, data: &[u8])
	{
		self.payload.extend(data.iter());
	}
}

impl fmt::Debug for Packet
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		let mut bytes = String::with_capacity(3*self.len());
		for i in 0..self.payload.len() {
			bytes.push_str(&format!(" {:02X}", self.payload[i]));
		}

        write!(f, "{}", bytes)
    }
}

impl Header
{
	pub fn new() -> Self
	{
		let data = Vec::with_capacity(20);
		Header{data}
	}

	pub fn with_capacity(capacity: usize) -> Self
	{
		let data = Vec::with_capacity(capacity);
		Header{data}
	}

	pub fn push8(&mut self, data: u8)
	{
		self.data.push(data);
	}

	pub fn push16(&mut self, data: u16)
	{
		self.data.push((data >> 8) as u8);
		self.data.push((data & 0xFF) as u8);
	}

	pub fn push32(&mut self, data: u32)
	{
		self.data.push((data >> 24) as u8);
		self.data.push(((data >> 16) & 0xFF) as u8);
		self.data.push(((data >> 8) & 0xFF) as u8);
		self.data.push((data & 0xFF) as u8);
	}

	pub fn push_bytes(&mut self, data: &[u8])
	{
		self.data.extend(data);
	}
}

pub fn push_ipv4(packet: &mut Packet)
{
	let payload_len = packet.len();
	let mut header = Header::with_capacity(20);

	let b = 0x45;						// version + IHL (we don't support options so length is fixed)
	header.push8(b);

	header.push8(20);

	let hw = 20 + payload_len;			// total length
	header.push16(hw as u16);

	header.push16(21);	// identification
	header.push16(23);

	packet.push_header(&header);
}

pub fn push_mac(packet: &mut Packet)
{
	let mut header = Header::with_capacity(30);

	let hw = 0b1000_10_00;		// frame control, see 9.2.4.1
	header.push16(hw);

	let addr = [1, 2, 3, 4, 5, 6];
	for &b in addr.iter() {	// address 1, see 9.3.2.1
		header.push8(b);
	}

	for &b in addr.iter() {	// address 2
		header.push8(b);
	}

	for &b in addr.iter() {// address 3
		header.push8(b);
	}

	header.push16(55);

	let hw = 0b111_0_00_0_000;	// QoS control, see 9.2.4.5.1
	header.push16(hw);

	packet.push_header(&header);

	let fcs = [0xD9, 0x58, 0xFB, 0xA8];
	println!("old packet = {:?}", packet);
	println!("pushing {:X} {:X} {:X} {:X} ", fcs[0], fcs[1], fcs[2], fcs[3]);
	packet.push_back_bytes(&fcs);

	println!("new packet = {:?}", packet);
}

fn main()
{
	let mut packet = Packet::new();
	push_ipv4(&mut packet);
	push_mac(&mut packet);
}
