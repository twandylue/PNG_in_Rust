pub struct BytePacketBuffer {
    pub buf: Vec<u8>,
    pos: usize,
    signature: [u8; 8],
}

pub const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

impl BytePacketBuffer {
    pub fn new() -> Self {
        BytePacketBuffer {
            buf: Vec::new(),
            pos: 0,
            signature: PNG_SIGNATURE,
        }
    }

    /// PNG Signature
    pub fn signature(&self) -> [u8; 8] {
        self.signature
    }

    /// Current position within buffer
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Step the buffer position forward a specific number of steps
    pub fn step(&mut self, steps: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.pos += steps;

        Ok(())
    }

    /// Change the buffer position
    pub fn seek(&mut self, pos: usize) -> Result<(), ()> {
        self.pos = pos;

        Ok(())
    }

    /// Read a single byte and move the position one step forward
    pub fn read(&mut self) -> Result<u8, Box<dyn std::error::Error>> {
        if self.pos >= self.buf.len() {
            return Err("End of buffer".into());
        }

        let res = self.buf[self.pos];
        self.pos += 1;
        Ok(res)
    }

    /// Get a single byte, without changing the buffer position
    pub fn get(&self, pos: usize) -> Result<u8, Box<dyn std::error::Error>> {
        if pos >= self.buf.len() {
            return Err("End of buffer".into());
        }
        Ok(self.buf[pos])
    }

    /// Get a range of bytes
    pub fn get_range(&self, start: usize, len: usize) -> Result<&[u8], Box<dyn std::error::Error>> {
        if start + len > self.buf.len() {
            return Err("End of buffer".into());
        }
        Ok(&self.buf[start..start + len])
    }

    /// Read four bytes, stepping four steps forward
    pub fn read_u32(&mut self) -> Result<u32, Box<dyn std::error::Error>> {
        let res = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);

        return Ok(res);
    }

    /// Read eight bytes, stepping four steps forward
    pub fn read_u64(&mut self) -> Result<u64, Box<dyn std::error::Error>> {
        let res = ((self.read()? as u64) << 56)
            | ((self.read()? as u64) << 48)
            | ((self.read()? as u64) << 40)
            | ((self.read()? as u64) << 32)
            | ((self.read()? as u64) << 24)
            | ((self.read()? as u64) << 16)
            | ((self.read()? as u64) << 8)
            | ((self.read()? as u64) << 0);

        return Ok(res);
    }
}
