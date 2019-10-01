
# Title

Rust: iterate faster

# LINQ

```csharp
float SumSqLinq(IEnumerable<float> xs)
{
  return xs.Sum(x => x * x);
}
```
```csharp
float SumSqLoop(IEnumerable<float> xs)
{
  var sumSq = 0.0;
  foreach (var x in xs)
  {
    sumSq += x * x;
  }
  return sumSq;
}
```
```csharp
float DotProductLinq(IEnumerable<float> xs, IEnumerable<float> ys)
{
  return xs.Zip(ys, (x, y) => x * y).Sum();
}
```
```csharp
float DotProductLoop(IEnumerable<float> xs, IEnumerable<float> ys)
{
    var result = 0.0;
    for (var i = 0; i < xs.Count(); ++i)
    {
        result += xs[i] * ys[i];
    }
    return result;
}
```
```csharp
float SumSlicesLinq(IEnumerable<float> xs, IEnumerable<float> ys)
{
    var sum = 0.0;
    foreach (var x in xs)
    {
      sum += x;
    }
    foreach (var y in ys)
    {
      sum += y;
    }
    return sum;
}
```
```csharp
float SumSlicesLoop(IEnumerable<float> xs, IEnumerable<float> ys)
    return xs.Concat(ys).Sum();
}
```

# Iterators

## Notes

## Slide

```rust
fn sum_sq_iter(xs: &[f32]) -> f32 {
    xs.iter().map(|x| x * x).sum()
}
```
```rust
fn sum_sq_loop(xs: &[f32]) -> f32 {
    let mut sum_sq = 0.0;
    for x in xs {
        sum_sq += x * x;
    }
    sum_sq
}
```
```rust
fn dot_product_iter(xs: &[f32], ys: &[f32]) -> f32 {
    xs.iter().zip(ys).map(|(x, y)| x * y).sum()
}
```
```rust
fn dot_product_loop(xs: &[f32], ys: &[f32]) -> f32 {
    let mut result = 0f32;
    for i in 0..xs.len() {
        result += xs[i] * ys[i];
    }
    result
}
```
```rust
fn sum_slices_iter(xs: &[f32], ys: &[f32]) -> f32 {
    let mut sum = 0.0;
    for x in xs.iter() {
        sum += x;
    }
    for y in ys.iter() {
        sum += y;
    }
    sum
}
```
```rust
fn sum_slices_loop(xs: &[f32], ys: &[f32]) -> f32 {
    xs.iter().chain(ys).sum()
}
```

# Performance

Rust using built in bencher.
TODO: notes on enumerable perf. numbers are from benchmark dot net
TODO: table of results! showing Rust vs C#, loop vs iter/linq

first slide: just C# results. Second slide: add Rust results

# Enums and matching

## Notes

... before digging deeper we need to introduce some Rust concepts ...
TODO: notes for enums

## Slide

```rust
enum Option<T> {
  Some(T),
  None,
}
```

```rust
let x = Some(7);
let y = match x {
  Some(n) => n + 2,
  None => 4
};
```

# Traits

## Notes

TODO: notes for traits

## Slide

```rust
trait Add<Rhs = Self> {
  type Output;

  fn add(self, rhs: Rhs) -> Self::Output;
}
```

```rust
struct Point {
  x: i32,
  y: i32,
}

impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}
```

```rust
fn add_three<A: Add>(x: A, y: A, z: A) -> A::Output {
  x.add(y).add(z) // or x + y + z
}
```

# Dyn trait

## Notes

TODO: notes for dyn trait

## Slide

```rust
dyn trait vs trait bounds

```

# Iterator

## Notes

TODO: notes for iterator

## Slide

```rust
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;

  // ... nth, map, filter, collect, ...
}
```
... todo: examples

# Why iterators are fast, part 1: monomorphisation

(show some x86)

# Why iterators are fast, part 2: LLVM

(inlining, loop invariant motion, autovectorisation)

# Why iterators are fast, part 3: try_fold

naive chain vs actual chain
(show more x86)

# Why LINQ is slow, part 1: function pointers

TODO

# Why LINQ is slow, part 2: no inlining, worse optimiser

TODO

# Can we copy the Rust approach in C#?

TODO

# Bonus slide 1: data race in parallel iterator

TODO

# Bonus slide 2: closures

TODO
