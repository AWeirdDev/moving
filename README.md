# 📦 `moving`

Make elements of an array or a vector movable with some simple magic.

## Vec -> array

Move elements of a vector an array.
```rust
use moving::move_vec_to_array;

let v = vec![0, 1, 2, 3, 4];  // 5 items
let arr = move_vec_to_array::<i16, 5>(v).unwrap();

assert_eq!(arr, [0, 1, 2, 3, 4]);
```

## Movable vectors

To make elements of an array or a vector movable, while the size is unknown, use `movable`:
```rust
use moving::{ MovableVec, movable };

let v = vec![0, 1, 2, 3, 4];  // 5 items
let arr = [0, 1, 2, 3, 4];

let mvv: MovableVec<i32> = movable(v);
let mvv_arr: MovableVec<i32> = movable(arr);
```

Alternatively, you can use `ToMovable`:
```rust
use moving::ToMovable;

some_vec.to_movable();
some_arr.to_movalbe();
```

## Movable arrays

To make elements of an array or a vector movable, while the size is known, use `nmovable` (notice the prefix "n"):
```rust
use moving::{ MovableArray, nmovable};

let v = vec![0, 1, 2, 3, 4];  // 5 items
let arr = [0, 1, 2, 3, 4];

let mvv: MovableArray<i32, 5> = nmovable(v).unwrap();
let mvv_arr: MovableArray<i32, 5> = nmovable(arr).unwrap();
```

Alternatively, you can use `ToNMovable`:
```rust
use moving::ToNMovable;

some_vec.to_nmovable()?;
some_arr.to_nmovable()?;
```
