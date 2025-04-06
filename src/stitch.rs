use core::mem::take;

use crate::*;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PacketHandler<T, const N: usize, const M: usize> {
    pub wrapped: T,
    pub buf: heapless::Vec<u8, N>,
    pub wbuf: heapless::Vec<u8, M>,
}
impl<T, const N: usize, const M: usize> PacketHandler<T, N, M> {
    pub fn new(a: T) -> Self {
        Self {
            wrapped: a,
            buf: Default::default(),
            wbuf: Default::default(),
        }
    }
}
impl<T: ErrorType, const N: usize, const M: usize> ErrorType for PacketHandler<T, N, M> {
    type Error = T::Error;
}
impl<T: ReadPacket<N>, const N: usize, const M: usize> embedded_io::Read
    for PacketHandler<T, N, M>
{
    fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut n = 0;
        loop {
            while self.buf.len() != 0 {
                if buf.len() == 0 {
                    return Ok(n);
                }
                let a = self.buf.remove(0);
                buf[0] = a;
                n += 1;
                buf = &mut buf[1..];
            }
            self.buf = self.wrapped.read_packet()?;
        }
    }
}
impl<T: AsyncReadPacket<N>, const N: usize, const M: usize> embedded_io_async::Read
    for PacketHandler<T, N, M>
{
    async fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut n = 0;
        loop {
            while self.buf.len() != 0 {
                if buf.len() == 0 {
                    return Ok(n);
                }
                let a = self.buf.remove(0);
                buf[0] = a;
                n += 1;
                buf = &mut buf[1..];
            }
            self.buf = self.wrapped.read_packet().await?;
        }
    }
}
impl<T: WritePacket<N>, const N: usize, const M: usize> embedded_io::Write
    for PacketHandler<T, M, N>
{
    fn write(&mut self, mut buf: &[u8]) -> Result<usize, Self::Error> {
        // let mut b: heapless::Vec<u8,N> = Default::default();
        let mut n = 0;
        loop {
            if self.wbuf.len() == N {
                self.wrapped.write_packet(take(&mut self.wbuf))?;
            }
            if buf.len() == 0 {
                return Ok(n);
            }
            let _ = self.wbuf.push(buf[0]);
            n += 1;
            buf = &buf[1..];
        }
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.wrapped.flush()
    }
}
impl<T: AsyncWritePacket<N>, const N: usize, const M: usize> embedded_io_async::Write
    for PacketHandler<T, M, N>
{
    async fn write(&mut self, mut buf: &[u8]) -> Result<usize, Self::Error> {
        let mut n = 0;
        loop {
            if self.wbuf.len() == N {
                self.wrapped.write_packet(take(&mut self.wbuf)).await?;
            }
            if buf.len() == 0 {
                return Ok(n);
            }
            let _ = self.wbuf.push(buf[0]);
            n += 1;
            buf = &buf[1..];
        }
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.wrapped.flush().await
    }
}
