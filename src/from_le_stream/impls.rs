use crate::FromLeStream;
use std::fmt::Debug;
use std::iter::once;
use std::mem::MaybeUninit;

impl FromLeStream for () {
    fn from_le_stream<T>(_: &mut T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Some(())
    }
}

impl FromLeStream for bool {
    fn from_le_stream<T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        bytes.next().map(|byte| byte != 0)
    }
}

impl FromLeStream for u8 {
    fn from_le_stream<T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = Self>,
    {
        bytes.next()
    }
}

impl FromLeStream for u16 {
    fn from_le_stream<T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for u32 {
    fn from_le_stream<T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for u64 {
    fn from_le_stream<T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for u128 {
    fn from_le_stream<T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for usize {
    fn from_le_stream<T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for i8 {
    fn from_le_stream<T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for i16 {
    fn from_le_stream<T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for i32 {
    fn from_le_stream<T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl FromLeStream for i64 {
    fn from_le_stream<T>(bytes: &mut T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        <[u8; size_of::<Self>()]>::from_le_stream(bytes).map(Self::from_le_bytes)
    }
}

impl<T, const SIZE: usize> FromLeStream for [T; SIZE]
where
    T: FromLeStream,
{
    fn from_le_stream<I>(bytes: &mut I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let mut result = [const { MaybeUninit::uninit() }; SIZE];

        for (index, item) in result.iter_mut().enumerate() {
            if let Some(elem) = T::from_le_stream(bytes) {
                item.write(elem);
            } else {
                // Drop all elements that we've initialized so far.
                for item in &mut result[0..index] {
                    #[allow(unsafe_code)]
                    // SAFETY: We counted the number of initialized elements,
                    // so it's safe to drop those.
                    unsafe {
                        item.assume_init_drop();
                    };
                }

                return None;
            }
        }

        #[allow(unsafe_code)]
        // SAFETY: At this point the array is fully initialized by the for loop above,
        // so it's safe to return it.
        Some(unsafe { result.as_ptr().cast::<Self>().read() })
    }
}

impl<T> FromLeStream for Option<T>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(bytes: &mut I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        bytes.next().map_or_else(
            || Some(None),
            |byte| T::from_le_stream(&mut once(byte).chain(bytes)).map(Some),
        )
    }
}

impl<T> FromLeStream for Vec<T>
where
    T: Debug + FromLeStream,
{
    fn from_le_stream<I>(bytes: &mut I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size: usize = usize::from_le_stream(bytes)?;
        let mut result = Self::with_capacity(size);

        for _ in 0..size {
            result.push(<T as FromLeStream>::from_le_stream(bytes)?);
        }

        Some(result)
    }
}

#[cfg(feature = "heapless")]
impl<T, const SIZE: usize> FromLeStream for heapless::Vec<T, SIZE>
where
    T: Debug + FromLeStream,
{
    fn from_le_stream<I>(bytes: &mut I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let size: usize = parse_size::<SIZE, I>(bytes)?;
        let mut result = Self::new();

        for _ in 0..size {
            result
                .push(<T as FromLeStream>::from_le_stream(bytes)?)
                .unwrap_or_else(|_| unreachable!());
        }

        Some(result)
    }
}

#[cfg(feature = "heapless")]
fn parse_size<const SIZE: usize, T>(bytes: &mut T) -> Option<usize>
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
