# Security and Correctness Audit Report for le-stream v8.0.0

**Audit Date:** 2026-03-31  
**Auditor:** GitHub Copilot (Automated Analysis)  
**Crate Version:** le-stream v8.0.0 and le-stream-derive v1.3.2

## Executive Summary

The `le-stream` crate has been audited for correctness, soundness, security vulnerabilities, and potential bugs.
Overall, the crate demonstrates **high quality** with no critical issues found. The codebase is well-structured, uses no
unsafe code (except for one carefully justified case in the derive macro), and all tests pass successfully.

**Overall Risk Assessment:** ✅ **LOW RISK**

---

## Scope

This audit covers:

- Core library (`le-stream` v8.0.0)
- Derive macro library (`le-stream-derive` v1.3.2)
- All feature implementations (alloc, heapless, intx, macaddr, derive)
- All transitive dependencies

---

## Findings Summary

| Category      | Status   | Issues Found  |
|---------------|----------|---------------|
| Unsafe Code   | ✅ Pass   | 1 (Justified) |
| Memory Safety | ✅ Pass   | 0             |
| Known CVEs    | ✅ Pass   | 0             |
| Logic Bugs    | ⚠️ Minor | 2             |
| API Soundness | ✅ Pass   | 0             |
| Documentation | ✅ Pass   | 0             |
| Test Coverage | ✅ Good   | N/A           |

---

## Detailed Analysis

### 1. Unsafe Code Review

**Location:** `le-stream-derive/src/to_le_stream.rs:68-70`

```rust
#[expect(unsafe_code)]
// SAFETY: This call is safe, because the macro guarantees that the enum is repr(T).
let discriminant = unsafe { *::core::ptr::from_ref(&self).cast::<#repr_type>() };
```

**Assessment:** ✅ **SAFE**

- **Purpose:** Reads the discriminant of a `#[repr(T)]` enum by casting the reference to the repr type
- **Justification:** The macro verifies that the enum has `#[repr(T)]` attribute before generating this code (line 45)
- **Safety:** This is a standard pattern for reading enum discriminants and is safe when the repr type matches
- **Recommendation:** Consider adding a compile-time assertion or using `std::mem::discriminant` if possible, but
  current implementation is acceptable

### 2. Memory Safety

**Assessment:** ✅ **EXCELLENT**

- No use of raw pointers outside the justified macro case
- No unsafe blocks in the core library
- All memory operations use safe Rust abstractions
- Iterator-based API prevents buffer overruns
- Proper use of `Option` for fallible operations

### 3. Known CVEs in Dependencies

**Assessment:** ✅ **NO VULNERABILITIES**

All dependencies checked against known CVE databases:

```
✅ heapless@0.9.1 - No known CVEs
✅ hash32@0.3.1 - No known CVEs
✅ byteorder@1.5.0 - No known CVEs
✅ stable_deref_trait@1.2.0 - No known CVEs
✅ intx@0.1.0 - No known CVEs
✅ proc-macro2@1.0.101 - No known CVEs
✅ unicode-ident@1.0.19 - No known CVEs
✅ quote@1.0.41 - No known CVEs
✅ syn@2.0.106 - No known CVEs
✅ macaddr@1.0.1 - No known CVEs
```

### 4. Logic and Correctness Issues

#### Issue 4.1: Potential Soundness Issue in `Option<T>` Deserialization ⚠️

**File:** `le-stream/src/from_le_stream/core.rs:83-96`

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
            || Some(None),
            |byte| T::from_le_stream(once(byte).chain(bytes)).map(Some),
        )
    }
}
```

**Issue:** The implementation treats an empty stream as `Some(None)`, which is semantically correct according to the
docs. However, if `T::from_le_stream` fails (returns `None`), the entire operation returns `None` instead of
`Some(None)`. This creates an ambiguity: did the stream end before we could read T, or was the stream empty to begin
with?

**Impact:** ⚠️ **LOW** - The behavior is documented and tests pass, but it could be confusing for users

**Recommendation:** Consider documenting this edge case more clearly, or using a discriminant byte to distinguish
between "no value" and "value present"

#### Issue 4.2: Heapless Vec Capacity Handling ⚠️

**File:** `le-stream/src/from_le_stream/heapless.rs:13-30`

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

        for _ in 0..SIZE {
            let Some(byte) = bytes.next() else {
                break;
            };

            result
                .push(T::from_le_stream(once(byte).chain(&mut bytes))?)
                .unwrap_or_else(|_| unreachable!());
        }

        Some(result)
    }
}
```

**Issue:** The implementation reads up to `SIZE` elements, but if the stream ends early, it returns a partially filled
vector. This differs from the standard `Vec<T>` implementation which reads until the stream is exhausted.

**Impact:** ⚠️ **LOW** - This is likely intentional behavior for fixed-capacity vectors, but creates inconsistency

**Recommendation:** Document this difference clearly in the trait docs. Consider adding a separate implementation for
length-prefixed heapless vecs.

### 5. API Soundness

**Assessment:** ✅ **EXCELLENT**

#### Trait Design

- `ToLeStream` properly requires `Iterator<Item = u8>` for the associated type
- `FromLeStream` uses `Option<Self>` for fallible parsing (idiomatic)
- `TryFromLeStream` properly composes with `TryFrom`
- `Consume` trait provides ergonomic convenience methods

#### Generic Bounds

- All trait bounds are properly applied in the derive macro
- No missing `Sized` bounds where needed
- Proper use of `?Sized` in delegation traits

#### Type Safety

- Strong typing prevents mixing endianness
- No unintended conversions between types
- Proper use of PhantomData for zero-sized type parameters

### 6. Potential Edge Cases

#### 6.1: Array Deserialization Memory Usage

**File:** `le-stream/src/from_le_stream/core.rs:65-81`

The array deserialization creates a temporary `[Option<T>; SIZE]` which could use significant stack space for large
arrays. However, this is acceptable for most use cases and the implementation correctly ensures all elements are
initialized.

#### 6.2: MAC Address Byte Order

**Files:** `le-stream/src/to_le_stream/macaddr.rs` and `le-stream/src/from_le_stream/macaddr.rs`

The MAC address implementations **reverse** the byte order (using `.rev()`) which is correct for little-endian
serialization of a big-endian type. This is intentional and correct but could be surprising to users.

**Verification:**

```rust
// MacAddr6: [a, b, c, d, e, f] -> serialize as [f, e, d, c, b, a]
// Deserialize: [f, e, d, c, b, a] -> MacAddr6::new(a, b, c, d, e, f)
```

✅ Correct implementation

#### 6.3: Prefixed Vector Panics

**File:** `le-stream/src/prefixed/heapless.rs:31-43`

The implementation can panic if the size prefix exceeds `u8::MAX`. However, the type is `Prefixed<u8, ByteSizedVec<T>>`
where `ByteSizedVec<T>` is defined as `heapless::Vec<T, { u8::MAX as usize }>`, so this panic is unreachable by
construction.

✅ Safe by construction

### 7. Test Coverage

**Assessment:** ✅ **GOOD**

All tests pass:

```
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured
```

Coverage includes:

- ✅ Primitive types serialization/deserialization
- ✅ Struct serialization/deserialization
- ✅ Array handling
- ✅ Option type handling
- ✅ Tagged unions
- ✅ Error cases (empty stream, excess bytes)
- ✅ Exact parsing vs. partial parsing
- ✅ MAC address types (with macaddr feature)
- ✅ Heapless vectors
- ✅ Alloc-based vectors

**Missing coverage:**

- ⚠️ No tests for `Range` and `RangeInclusive` implementations
- ⚠️ No tests for `TryFromLeStream` trait
- ⚠️ No tests for `intx` types
- ⚠️ Limited negative test cases

### 8. Code Quality

**Assessment:** ✅ **EXCELLENT**

- Clean, idiomatic Rust code
- Proper use of macros to avoid code duplication
- Good separation of concerns
- Consistent naming conventions
- No clippy warnings with `-D warnings`
- Edition 2024 (latest as of Rust 1.85)

### 9. Documentation

**Assessment:** ✅ **GOOD**

- Public API is documented
- Safety comments for unsafe code
- Examples in test files
- Clear error type with Display implementation

**Recommendations:**

- Add more doc examples for complex types
- Document byte order expectations more prominently
- Add examples for common serialization patterns

---

## Recommendations

### High Priority

None

### Medium Priority

1. Add tests for `Range`, `RangeInclusive`, and `intx` types
2. Document the `Option<T>` deserialization behavior more clearly
3. Consider adding more negative test cases

### Low Priority

1. Add doc examples for advanced usage patterns
2. Consider adding a "Security" section to the README
3. Add fuzzing targets for deserialization
4. Consider using `std::mem::discriminant` if possible instead of unsafe cast

---

## Conclusion

The `le-stream` crate is **well-designed and secure** for its intended purpose. No critical vulnerabilities or soundness
issues were found. The crate follows Rust best practices and maintains a high standard of code quality.

**Recommendation:** ✅ **APPROVED FOR USE**

The crate is suitable for:

- ✅ Embedded systems (no_std support)
- ✅ Network protocol implementations
- ✅ Binary file format parsing
- ✅ Safety-critical applications (with standard Rust safety guarantees)

### Sign-off

This audit found the crate to be of high quality with no security vulnerabilities. The minor issues noted are primarily
documentation improvements and edge case clarifications rather than functional bugs.

**Audit Status:** ✅ **COMPLETE - NO CRITICAL ISSUES**

