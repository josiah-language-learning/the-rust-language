# Data Types

- 2 subsets
    1. **scalar** 
    1. **compound** 
- Types can normally be inferred, but sometimes such as when parsing or casting,
  the type cannot be automagically inferred without an explicit annotation
  written by the programmer.
  
## Scalar Types

Represent a single value.

### 4 Principal kinds, with subkinds

1. **_integers_**: Whole numbers without any fractional/decimal portion.
    - **Integer Types**
        - `i8`, `u8`
        - `i16`, `u16`
        - `i32`, `u32`
        - `i64`, `u64`
        - `i128`, `u128`
        - `isize`, `usize` <- size of the CPU register of the current/target machine
    - **Integer Literal Representations**
        - **Decimal:** `98_222`
        - **Hex:** `0x17fae`
        - **Octal:** `0o277656`
        - **Binary:** `0b001_0111_1111_1010_1110`
        - **Byte** (`u8` only): `b'A'` *not 98222 since not representable in u8*
    - **Further Info**
        - `i32` is the default and is generally faster even on 64-bit archs.
        - `isize` & `usize` are primarily used to perform indexing on
           collections.
        - **Integer Overflow**
            - In *debug mode* only, the compiler can catch some instances of
              this typically runtime error.
            - In *release mode*, `rustc` performs *two's compliment wrapping*
              to represent `<the number> % <max data type size> - 1`.
                - e.g. max for a byte is 255.  If you put `256` in a byte variable
                  and compile in `--release` mode, the variable will hold the
                  value `0`, but if you put `257`, it would hold value `1`, etc.
                - This is considered bad practice.  **¡¡DON'T DO THIS!!**
1. **_floating-point numbers_**: Real numbers with a fractional/decimal portion.
    - **Floating Point Types**
        - `f32` *single precision*
        - `f64` *double precision: <- default floating-point type*
            - Default because most modern CPUs work with ~ the same performance
              and efficiency on `f64` as on `f32`, so it doesn't make sense to
              default to `f32`. 
                - This may not be the case for embedded and microcontroller
                  hardware
1. **_Booleans_**: `true` or `false`, one `byte` in size.
1. **_characters_**: Unicode Scalar Values which can represent more than ASCII
                     such as accented letters, non-English chars, emojis, etc.
                     Range from `U+0000` to `U+D7FF` & `U+E000` to `U+10FFFF`
                     inclusive.
    - Specified with `'`s (single-quotes)
    - Each char is 4 `bytes` in size.
    - It's not explicitly stated at this point in the book, but hinted at that
      `String`s are made-up of `char`s.
                     
#### Numeric Operations

All numeric types support the operations you'd expect, `+`, `-`, `*`, `/`, `%`.
These numeric operators follow the normal mathematical order of operations like
any sane, normal human being would expect.

There's also other operators such as `/=` type operation + assignment type
operators, bitwise operators, shift operators, etc.  They are listed in the
[book's appendix](https://doc.rust-lang.org/book/appendix-02-operators.html).
There's some interesting operators there that not all languages have.  It's
worth it to spend some time poking around and tinkering with these appendix
operators.

## Compound Types

*Compound Types* can group multiple values into one type.  In some other
languages such as C#, Java, and C++, these may be referred to as "containers".
In Rust, the 2 primitive compound types are tuples and arrays. Tuples may mix
types in the container, but arrays may not.  **Tuples and arrays both have a
fixed length in Rust.** Once declared, tuples may not grow or shrink in size
and, in general, they are more difficult to compose and decompose than arrays.
Tuples will allocate your data on the **_heap_**, arrays will allocate on the
**_stack_**.  In general, favor `vector` over `array`, as vectors can flex
in size and are more dynamic.  `vector` isn't covered in this chapter, however.
