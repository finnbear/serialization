# Optimizing Network Protocol Performance: Eliminating Endian Overhead

## Overview
Traditionally, network protocols use Big Endian for byte order, which imposes certain performance overheads when working with Little Endian architectures, such as ARM. This document outlines the inefficiencies caused by enforcing a specific endianness, explores the benefits of removing such constraints, and highlights an optimization strategy using custom macros for efficient packet encoding and decoding.

## The Problem with Fixed Endianness

1. **Performance Cost on Little Endian Architectures:**
   - Decoding and encoding packets in a fixed endianness require conversion operations on Little Endian systems.
   - These conversions result in negligible but measurable performance penalties. For example, even a tiny 0.0000000001% cost adds up in high-throughput systems.

2. **Complex Encoding/Decoding Logic:**
   - When endianness is enforced, fields within objects need to be individually encoded or decoded.
   - This introduces unnecessary complexity compared to simply copying an entire object to or from a buffer.

3. **Repeated Buffer Size Checks:**
   - Without a direct memory copy, buffer size checks must be performed for every individual field during encoding/decoding.
   - This results in redundant computations that scale with the number of fields.

## The Solution: Removing Endian Constraints
By removing strict endianness requirements in the network protocol, we can:

1. **Simplify Data Handling:**
   - Use padding-free objects that can be directly copied to and from buffers.

2. **Handshake with Endianness Information:**
   - During connection setup, the server can transmit its endianness to the client, enabling seamless interpretation without conversions.

3. **Leverage Custom Macros:**
   - Efficient macros can handle encoding/decoding while accounting for variable-sized fields (e.g., var-int) and padding.
   - These macros avoid redundant checks and focus on optimizing performance.

## Performance Gains
Using the described optimization techniques, encoding and decoding times have shown measurable improvements:

```rust
#[derive(Serializable)]
pub struct Foo {
    v2: u8,
    v1: Bar,
    v3: u64,
    v4: i32,
    v5: u16,
}

#[derive(Serializable)]
pub struct Bar {
    v1: i32,
}


```

### Before Optimization
```
           fastest       │ slowest       │ median        │ mean          
├─ decode  3.671 ns      │ 8.087 ns      │ 3.796 ns      │ 3.808 ns      
╰─ encode  1.921 ns      │ 3.671 ns      │ 2.004 ns      │ 2.409 ns      

```

### After Optimization
```
           fastest       │ slowest       │ median        │ mean         
├─ decode  2.004 ns      │ 2.254 ns      │ 2.088 ns      │ 2.093 ns     
╰─ encode  0.879 ns      │ 1.171 ns      │ 0.963 ns      │ 0.954 ns     

```

### Field-Specific Impact
Optimizations yield approximately **0.1 ns gain per 2-3 fields**, demonstrating significant benefits in systems with complex objects.

## Practical Implementation
### Why Not Use Serde?
Rust's Serde framework, while versatile, is not ideal for all scenarios. Specifically, it cannot serialize enum variant indices as var-ints, limiting its applicability in contexts requiring highly optimized serialization.

### Benefits of Custom Macros
- **Direct Memory Copy:** Avoid encoding/decoding logic for fixed-size fields.
- **Padding-Free Objects:** Ensure memory alignment without unnecessary padding.
- **Var-Int Handling:** Efficiently manage variable-sized fields with custom logic.

A macro implementation avoids overhead by:
- Checking buffer size only once per copy.
- Skipping padding and processing fields with variable sizes in binary encoding.

## Conclusion
By removing fixed endianness constraints and leveraging efficient macros, significant performance improvements can be achieved in packet encoding and decoding. This approach is especially beneficial for high-performance applications, such as game servers, where every nanosecond counts.

# Requirements
```rust
#![feature(const_trait_impl)]
#![feature(generic_const_exprs)]
#![feature(specialization)]

```
