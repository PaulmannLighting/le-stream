use crate::FromLeStream;

pub fn parse_size<const SIZE: usize, T>(bytes: &mut T) -> Option<usize>
where
    T: Iterator<Item = u8>,
{
    if u8::try_from(SIZE).is_ok() {
        <u8 as FromLeStream>::from_le_stream(bytes).map(|size| size as usize)
    } else if u16::try_from(SIZE).is_ok() {
        <u16 as FromLeStream>::from_le_stream(bytes).map(|size| size as usize)
    } else if u32::try_from(SIZE).is_ok() {
        <u32 as FromLeStream>::from_le_stream(bytes).map(|size| size as usize)
    } else {
        <u64 as FromLeStream>::from_le_stream(bytes)
            .map(|size| usize::try_from(size).expect("usize exceeds u64"))
    }
}
