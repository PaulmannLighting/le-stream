# Detailed Technical Findings - le-stream Audit

## 1. Unsafe Code Analysis

### Finding 1.1: Enum Discriminant Reading via Pointer Cast

**Location:** `le-stream-derive/src/to_le_stream.rs:68-70`

**Code:**

```rust
#[expect(unsafe_code)]
// SAFETY: This call is safe, because the macro guarantees that the enum is repr(T).
let discriminant = unsafe { *::core::ptr::from_ref(&self).cast::<#repr_type>() };
```

**Analysis:**

- **Purpose:** Extract the discriminant value from a `#[repr(T)]` enum
- **Safety Invariant:** The enum must have `#[repr(T)]` attribute matching the cast type
- **Verification:** The macro checks for `#[repr(T)]` at line 45: `let repr_type = repr_type.expect("\`#[repr(T)]\` is
  required");`
- **Memory Layout:** For `#[repr(T)]` enums, the discriminant is stored as the first field with type `T`
- **Alignment:** The cast preserves alignment requirements since we're casting from a reference
- **Aliasing:** Read-only access through immutable reference - no aliasing issues

**Verdict:** ✅ **SAFE** - The safety invariants are properly maintained by compile-time checks

**Alternative Approaches:**

1. Use `std::mem::discriminant()` - but this returns an opaque type, not the actual value
2. Match on all variants - impractical for derive macro generation
3. Current approach is the standard pattern used in many proc macros

---

## 2. Semantic Issues

### Finding 2.1: Option<T> Deserialization Ambiguity

**Location:** `le-stream/src/from_le_stream/core.rs:83-96`

**Code:**

```rust
impl<T> FromLeStream for Option<T>
where
    T: FromLeStream,
{
    /// This is guaranteed to always return `Some(Option<T>)`.
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        bytes.next().map_or_else(
            || Some(None),  // Empty stream -> Some(None)
            |byte| T::from_le_stream(once(byte).chain(bytes)).map(Some),
            // ^ If T::from_le_stream fails -> None (not Some(None))
        )
    }
}
```

**Problem:**
The function returns `None` in two distinct scenarios:

1. Stream ends before we can read all of `T` → returns `None`
2. We successfully read nothing → returns `Some(None)`

But there's no way to distinguish between "partial read of T failed" vs "empty stream".

**Test Case:**

```rust
// This works as expected
let bytes = [];
let result: Option<Option<u8>> = Option::from_le_stream(bytes.into_iter());
assert_eq!(result, Some(None)); // ✅ Empty stream = no value

// This also returns None, but for a different reason
let bytes = [0xAB];
let result: Option<Option<u16>> = Option::from_le_stream(bytes.into_iter());
assert_eq!(result, None); // ⚠️ Failed to read u16 (needs 2 bytes, got 1)
```

**Impact:** Users cannot distinguish between:

- "The stream was empty, so the Option is None"
- "The stream had data but parsing failed"

**Severity:** ⚠️ **LOW** - The documentation states "This is guaranteed to always return `Some(Option<T>)`" which is
technically violated when `T::from_le_stream` fails.

**Recommendations:**

1. Document that `None` is returned when `T` fails to parse
2. Consider returning `Some(None)` even when `T` fails (questionable semantics)
3. Use an explicit discriminant byte (breaking change)

---

### Finding 2.2: Heapless Vec Size Semantics Inconsistency

**Location:** `le-stream/src/from_le_stream/heapless.rs:9-30`

**Code:**

```rust
impl<T, const SIZE: usize> FromLeStream for Vec<T, SIZE>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let mut result = Self::new();

        for _ in 0..SIZE {  // ⚠️ Reads UP TO SIZE elements
            let Some(byte) = bytes.next() else {
                break;  // ⚠️ Early return with partial data
            };

            result
                .push(T::from_le_stream(once(byte).chain(&mut bytes))?)
                .unwrap_or_else(|_| unreachable!());
        }

        Some(result)  // ⚠️ Returns partially filled vector
    }
}
```

**Compare with alloc Vec:**

```rust
impl<T> FromLeStream for Vec<T>
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let mut result = Self::new();

        while let Some(byte) = bytes.next() {  // ⚠️ Reads UNTIL exhaustion
            result.push(T::from_le_stream(once(byte).chain(&mut bytes))?);
        }

        Some(result)
    }
}
```

**Behavioral Difference:**

| Input                   | alloc::Vec            | heapless::Vec&lt;T, 5> |
|-------------------------|-----------------------|------------------------|
| `[1, 2, 3]`             | `vec![1, 2, 3]`       | `vec![1, 2, 3]`        |
| `[1, 2, 3, 4, 5, 6, 7]` | `vec![1,2,3,4,5,6,7]` | `vec![1,2,3,4,5]`      |
| `[1, 2]`                | `vec![1, 2]`          | `vec![1, 2]`           |

**Problem:**

- `alloc::Vec` reads until the stream is exhausted
- `heapless::Vec` reads up to SIZE elements, even if more are available
- This breaks the property that `T::to_le_stream().collect()` can be round-tripped

**Test Case:**

```rust
let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];
let serialized: Vec<u8> = data.to_le_stream().collect();

// Try to deserialize into heapless::Vec<u8, 5>
let deserialized = heapless::Vec::<u8, 5>::from_le_stream(serialized.into_iter());
// deserialized = Some(vec![1, 2, 3, 4, 5])
// Lost bytes: [6, 7] ⚠️
```

**Severity:** ⚠️ **MEDIUM** - This is a correctness issue that could lead to data loss

**Recommendations:**

1. Document this behavior clearly in the trait impl docs
2. Consider adding a `from_le_stream_exact` variant that requires exact size match
3. For consistency, either:
    - Make heapless Vec read until exhaustion (may fail if capacity exceeded)
    - Make alloc Vec also use a size prefix (breaking change)

---

## 3. Edge Cases

### Finding 3.1: Large Array Stack Usage

**Location:** `le-stream/src/from_le_stream/core.rs:65-81`

**Code:**

```rust
impl<T, const SIZE: usize> FromLeStream for [T; SIZE]
where
    T: FromLeStream,
{
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let mut array = [const { None }; SIZE];  // ⚠️ SIZE * size_of::<Option<T>>() bytes

        for element in &mut array {
            element.replace(T::from_le_stream(&mut bytes)?);
        }

        Some(array.map(|element| element.expect("All elements are initialized.")))
    }
}
```

**Problem:**
For large arrays or large `T`, this allocates `SIZE * size_of::<Option<T>>()` bytes on the stack.

**Example:**

```rust
let arr: [u64; 1000] = [0; 1000];
// Deserialization uses ~16KB of stack space (1000 * 16 bytes for Option<u64>)
```

**Severity:** ⚠️ **LOW** - This is acceptable for most use cases, but could cause stack overflow in embedded systems
with limited stack

**Mitigation:**

- Embedded systems should use `heapless::Vec` or smaller arrays
- Stack overflow would be caught in debug builds
- This is a known limitation of const generic arrays in Rust

---

### Finding 3.2: MAC Address Byte Order

**Location:** `le-stream/src/to_le_stream/macaddr.rs:10-24`

**Code:**

```rust
impl ToLeStream for MacAddr6 {
    type Iter = Rev<IntoIter<u8, 6>>;

    fn to_le_stream(self) -> Self::Iter {
        self.into_array().into_iter().rev()  // ⚠️ Reverses byte order
    }
}
```

**Rationale:**
MAC addresses are conventionally represented in big-endian (network byte order).
The crate serializes to little-endian, so the bytes are reversed.

**Verification:**

```rust
use macaddr::MacAddr6;

let mac = MacAddr6::new(0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF);
// Standard representation: AA:BB:CC:DD:EE:FF

let bytes: Vec<u8> = mac.to_le_stream().collect();
// Little-endian: [0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA]

let mac2 = MacAddr6::from_le_stream(bytes.into_iter()).unwrap();
assert_eq!(mac, mac2); // ✅ Round-trip works
```

**Severity:** ✅ **CORRECT BEHAVIOR** - This is the expected behavior for little-endian serialization

**Documentation Note:** This might be surprising to users who expect MAC addresses to serialize in their "natural"
big-endian order.

---

## 4. Potential Panics

### Finding 4.1: Heapless Vector Push Unwrap

**Location:** `le-stream/src/from_le_stream/heapless.rs:24-26`

**Code:**

```rust
result
    .push(T::from_le_stream(once(byte).chain(&mut bytes))?)
    .unwrap_or_else(|_| unreachable!());
```

**Analysis:**
The `push` can only fail if the vector is at capacity. However, the loop is bounded by `for _ in 0..SIZE`, and the
vector has capacity `SIZE`, so this is unreachable.

**Proof:**

```
Loop iteration i:
  - result.len() = i (invariant)
  - i < SIZE (loop bound)
  - result.capacity() = SIZE
  - Therefore: result.len() < result.capacity()
  - Therefore: push cannot fail
```

**Verdict:** ✅ **SAFE** - The unreachable is justified by the loop invariant

---

### Finding 4.2: Prefixed Heapless Vector Length

**Location:** `le-stream/src/prefixed/heapless.rs:38-40`

**Code:**

```rust
for _ in 0..size {
    data.push(T::from_le_stream(&mut bytes)?)
        .unwrap_or_else(|_| unreachable!("Size cannot exceed vector capacity."));
}
```

**Analysis:**

- `size` has type `u8` (max value 255)
- `data` has type `heapless::Vec<T, { u8::MAX as usize }>` (capacity 255)
- Therefore: `size <= 255 == capacity`

**Proof:**

```
size: u8 -> size ∈ [0, 255]
capacity = u8::MAX as usize = 255
Therefore: size <= capacity
```

**Verdict:** ✅ **SAFE** - The unreachable is guaranteed by type system

---

## 5. Documentation Issues

### Finding 5.1: Missing Test Coverage

**Missing Tests:**

1. `Range<T>` and `RangeInclusive<T>` serialization/deserialization
2. `TryFromLeStream` trait usage
3. `intx` types (U24, U40, etc.)
4. Negative cases for derive macros
5. Endianness verification tests

**Example Test Case to Add:**

```rust
#[test]
fn test_range_roundtrip() {
    let range: Range<u32> = 10..20;
    let bytes: Vec<u8> = range.to_le_stream().collect();
    let range2: Range<u32> = Range::from_le_stream(bytes.into_iter()).unwrap();
    assert_eq!(range, range2);
}
```

---

## 6. API Design Considerations

### From TODO.md:

1. **Remove `FromLeStreamTagged`** - Now redundant since enums can derive `FromLeStream`
2. **Revisit `TryFromLeStream`** - Consider if the API is ergonomic
3. **Revisit `Prefixed`** - Consider if it belongs in this crate or should be separate

**Analysis:**

#### 6.1 FromLeStreamTagged Redundancy

```rust
// Old way with FromLeStreamTagged
impl FromLeStreamTagged for MyEnum { ... }

// New way with derive
#[derive(FromLeStream)]
#[repr(u16)]
enum MyEnum { ... }
```

**Recommendation:** Mark `FromLeStreamTagged` as deprecated in favor of the derive macro.

---

## 7. Summary of Findings

| ID  | Type          | Severity | Status       |
|-----|---------------|----------|--------------|
| 1.1 | Unsafe Code   | Info     | ✅ Safe       |
| 2.1 | Semantics     | Low      | ⚠️ Document  |
| 2.2 | Correctness   | Medium   | ⚠️ Document  |
| 3.1 | Performance   | Low      | ✅ Acceptable |
| 3.2 | Documentation | Info     | ✅ Correct    |
| 4.1 | Panic         | Info     | ✅ Safe       |
| 4.2 | Panic         | Info     | ✅ Safe       |
| 5.1 | Testing       | Low      | ⚠️ Improve   |
| 6.1 | API Design    | Info     | 📝 TODO      |

**Overall Assessment:** The crate is well-designed and secure. The issues found are primarily documentation and edge
case clarifications rather than functional bugs or security vulnerabilities.

