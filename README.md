### Learning basics in Rust language

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