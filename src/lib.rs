#![no_std]

use embedded_io::ErrorType;
pub trait ReadPacket<const N: usize>: ErrorType {
    fn read_packet(&mut self) -> Result<heapless::Vec<u8, N>, Self::Error>;
}
pub trait AsyncReadPacket<const N: usize>: ErrorType {
    async fn read_packet(&mut self) -> Result<heapless::Vec<u8, N>, Self::Error>;
}
pub trait WritePacket<const N: usize>: ErrorType{
    fn write_packet(&mut self, x: heapless::Vec<u8,N>) -> Result<(),Self::Error>;
    fn flush(&mut self) -> Result<(),Self::Error>;
}
pub trait AsyncWritePacket<const N: usize>: ErrorType{
    async fn write_packet(&mut self, x: heapless::Vec<u8,N>) -> Result<(),Self::Error>;
    async fn flush(&mut self) -> Result<(),Self::Error>;
}
pub mod stitch;