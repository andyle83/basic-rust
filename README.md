### Learning basics in Rust language

This Repo includes some dummy code that I tried to practice during reading the Book `Programming Rust 2nd`
It helps me to have a better memo by 
- Trying some syntax
- Trying to solve some problem with Rust
- Participate on some blockchain project that use Rust

Reading content (updating...)

1. System type
   - Numbers (Int, Float, Double, Byte)
   - Characters
   - Tuples
   - Pointer: represent memory addresses
     - References (Immutable by default)
     - Boxes (Heap)
     - Unsafe Pointer
   - Collection
     - Array `[T; N]`: fixed size determined at compile time
     - Vector Vec<T>: dynamic allocated
     - Shared slice & Mutable slice (Q: what is `shared` meaning)
       - Shared because it is just a reference, not owner
       - Slide is just a reference point to sub-set of data in a collection like array / vector
   - String
     - Literal string
     - Byte string ~ slice of byte array
     - String memory (make difference between string - ownership / and string slide - borrow)
   
2. Ownership Rule
3. Reference
4. Expression
5. Error Handling
6. Crates & Module
7. Structs
8. Enums & Pattern
9. Traits & Generic
