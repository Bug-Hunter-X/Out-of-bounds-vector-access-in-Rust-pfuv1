# Out-of-bounds Vector Access in Rust
This repository demonstrates a common error in Rust: accessing a vector element using an index that is out of bounds.  This often leads to a runtime panic.  The solution demonstrates safe ways to handle potential index errors.

**Bug:** The `bug.rs` file contains code that attempts to access an element beyond the valid index range of a vector.  This causes a panic.

**Solution:** The `bugSolution.rs` file shows how to use the `get()` method with error handling to avoid panics.  It checks the index before accessing the element, preventing out-of-bounds errors.