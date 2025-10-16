# 🦀 RUST OWNERSHIP — COMPLETE WALKTHROUGH

> A complete hands-on walkthrough of **Rust’s Ownership System**, explained in simple, visual, and practical terms.  
> Based on [*The Rust Programming Language Book* — Chapter 4].

Ownership is the foundation of Rust’s _memory safety_. It eliminates common bugs such as:

- Use-after-free
- Double-free
- Data races  
  ...and it does so **without a garbage collector**.

---

## TABLE OF CONTENTS

1. [Introduction](#-introduction)
2. [The Ownership Rules](#-ownership-rules)
3. [Variable Scope](#-1-variable-scope)
4. [The `String` Type and Heap Allocation](#-2-the-string-type-and-heap-allocation)
5. [Move Semantics](#-3-move-semantics)
6. [Deep Copy with `clone()`](#-4-deep-copy-with-clone)
7. [Stack Data and the Copy Trait](#-5-stack-data-and-the-copy-trait)
8. [Ownership and Functions](#-6-ownership-and-functions)
9. [Returning Ownership](#-7-returning-ownership)
10. [Borrowing with References](#-8-borrowing-with-references)
11. [Mutable References](#-9-mutable-references)
12. [Preventing Data Races](#-10-preventing-data-races)
13. [Dangling References](#-11-dangling-references)
14. [Ownership Flow Diagrams](#-ownership-flow-diagrams)
15. [Summary](#-summary)

---

## INTRODUCTION

Rust’s **ownership system** manages memory through a set of compile-time rules.  
There is no garbage collector or manual `malloc`/`free` — Rust ensures **safe memory handling automatically**.

### Why It Matters

Ownership ensures:

- No null pointers
- No dangling references
- No data races (even in multi-threaded code)
- Memory safety without runtime overhead

---

## ⚙️ OWNERSHIP RULES

1. Every value in Rust has **exactly one owner**.
2. When the owner goes **out of scope**, the value is **dropped** (freed).
3. Ownership can be **moved** or **borrowed**, but not both at the same time.

---

## 1. VARIABLE SCOPE

```rust
fn main() {
    let s = "hello"; // string literal stored in binary
    println!("{}", s);
} // s goes out of scope — nothing special happens
```
