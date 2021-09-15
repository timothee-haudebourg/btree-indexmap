# Indexed B-Tree.

Pure Rust B-Tree data structures that preserve insertion order.
This crate provides two collections `IndexSet` and `IndexMap` built on top of
the [`generic-btree`](https://crates.io/crates/generic-btree) crate where each
entry is indexed and is iterated by ascending index,
preserving the order of insertion in the set/map.
However, two sets/maps are considered equals if they contains the same entries,
regardless of their index.

## Background

This is inspired by the [`indexmap`](https://crates.io/crates/indexmap) crate that provides similar data-structures
based on a hash table. Even if using a B-Tree is generally slower
(constant time accesses for a hash table versus log time accesses for a B-Tree based implementation),
using a B-Tree allows us to implement the `Hash`, `PartialOrd` and `Ord` traits on the data-structure.
Again, note that the hash value associated to the data-structure is independent of the index of each entry.

## Implementation

Under the hood, the indexed B-Tree is composed of a linked list of entries,
an array of references to the linked list items preserving the insertion order and
a B-Tree of references to the linked list items preserving the logical order.

```text
 Index (vec) ┆   Data (list)  ┆   Order (B-Tree)
┌────────┐   ┆   ┌────────┐   ┆   ┌────────┐
│   @1   ┼──┐┆┌─>│ Item 1 │<──────┼   @0   │
│        │  │┆│  └───┼────┘   ┆   │        │
│        │  │┆│  ┌───┼────┐   ┆   │        │<─────┐
│   @0   ┼────┘─>│ Item 0 │<─┐┆┌──┼   @2   │      │
│        │   ┆   └───┼────┘  │┆│  └────────┘      │
│        │   ┆   ┌───┼────┐  │┆│              ┌────────┐
│   @3   ┼──┐┆┌─>│ Item 4 │<───┘──────────────┼   @1   │
│        │  │┆│  └───┼────┘   ┆               └────────┘
│        │  │┆│  ┌───┼────┐   ┆   ┌────────┐      │
│   @4   ┼──┐─┼─>│ Item 2 │<─┐┆┌──┼   @4   │      │
│        │  │┆│  └───┼────┘  │┆│  │        │<─────┘
│        │  │┆│  ┌───┼────┐  │┆│  │        │
│   @2   ┼────┘─>│ Item 3 │<───┘──┼   @3   │
└────────┘   ┆   └────────┘   ┆   └────────┘
```

Note that all nodes of the linked list and the B-Tree are stored in a single (one for each) linear memory array to improve locality and reduce memory allocations.

## Performance

The `IndexSet` and `IndexMap` data-structure generally deliver the same performances
as a regular B-Tree.
The following table show the differences of mean time complexity
between this implementation and the hash-table based `indexmap` crate.

| Operation               | `indexed-btree` | `indexmap` |
|-------------------------|-----------------|------------|
| Lookup by key           | O(log n)        | O(1)       |
| Lookup by index         | O(1)            | O(1)       |
| Insert                  | O(log n)        | O(1)       |
| Remove by key           | O(log n)        | O(1)       |
| Remove by index (swap)  | O(log n)        | O(1)       |
| Remove by index (shift) | O(n)            | O(n)       |

## Composability

The advantage of the B-Tree based approach is that it
provides implementation to more traits compared to the
the hash-map based approach, which means that it can be used in more contexts.

| Trait      | `indexed-btree` | `indexmap`    |
|------------|-----------------|---------------|
| Hash       | yes             | no (not yet?) |
| PartialOrd | yes             | no            |
| Ord        | yes             | no            |