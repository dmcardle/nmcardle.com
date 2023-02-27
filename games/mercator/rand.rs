use std::fs::File;
use std::io::Read;

pub trait RandomStream {
    fn read_u8(&mut self) -> u8;

    fn read_usize(&mut self) -> usize {
        const N: usize = std::mem::size_of::<usize>();
        let mut bytes = [0u8; N];
        for out in bytes.iter_mut() {
            *out = self.read_u8();
        }
        usize::from_ne_bytes(bytes)
    }
}

/// Fisher-yates shuffle.
pub fn shuffle<T>(r: &mut dyn RandomStream, buf: &mut [T]) {
    for i in (1..buf.len()).rev() {
        let k = r.read_usize();
        buf.swap(i, k % i);
    }
}

/// Get a RandomStream for the current platform.
///
/// TODO: Support non-Linux platforms.
pub fn get_system_random_stream() -> std::io::Result<Box<dyn RandomStream>> {
    Ok(Box::new(RandomStreamLinux::new()?))
}

const RAND_BUF_SIZE: usize = 1024;

struct RandomStreamLinux {
    file: File,
    buf: [u8; RAND_BUF_SIZE],
    remaining: usize,
}

impl RandomStreamLinux {
    fn new() -> std::io::Result<RandomStreamLinux> {
        Ok(RandomStreamLinux {
            file: File::open("/dev/urandom")?,
            buf: [0u8; RAND_BUF_SIZE],
            remaining: 0,
        })
    }
}

impl RandomStream for RandomStreamLinux {
    fn read_u8(&mut self) -> u8 {
        if self.remaining > 0 {
            let byte = self.buf[RAND_BUF_SIZE - self.remaining];
            self.remaining -= 1;
            byte
        } else {
            self.remaining = self.file.read(&mut self.buf).expect("Must read from file");
            self.read_u8()
        }
    }
}

pub struct RandomStreamForTest {
    counter: u8,
}

impl RandomStreamForTest {
    pub fn new() -> Self {
        RandomStreamForTest { counter: 0 }
    }
}

impl RandomStream for RandomStreamForTest {
    fn read_u8(&mut self) -> u8 {
        self.counter.wrapping_add(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut stream = get_system_random_stream().expect("Should get system random stream");
        stream.read_u8();
        stream.read_usize();
    }

    #[test]
    fn test_exceed_buf_size() {
        let mut stream = get_system_random_stream().expect("Should get system random stream");
        for _ in 0..RAND_BUF_SIZE + 1 {
            stream.read_u8();
        }
    }

    #[test]
    fn test_shuffle() {
        let mut stream = get_system_random_stream().expect("Should get system random stream");
        let original_numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let mut shuffled_numbers = original_numbers;
        shuffle(stream.as_mut(), &mut shuffled_numbers);
        // Test that the shuffled list is not equal to the original list. If we
        // are astronomically unlucky, shuffling will produce the original
        // permutation and this test will fail.
        assert_ne!(shuffled_numbers, original_numbers);
    }
}
