#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

// -------------
// Rust: iterate faster
// -------------
//
// Abstract:
// LINQ is ergonomic but slow. Rust iterators have a similar
// look and feel but are much faster. Why are iterators faster,
// and what stops us from using the same ideas in C#?
//
// This talk assumes you know C#, but does not require any prior Rust
// knowledge. (Howver, it does not attempt to give a general introduction
// to Rust. For example, the borrow checker barely gets a mention.)
//
// -------------
// Outline
// -------------
// Slide 1: sum_sq, dot_product, concat loop and LINQ in C#, with perf numbers (note: using BenchmarkDotNet)
// Slide 2: sum_sq, dot_product, chain loop and iterators in Rust, with perf numbers (note: using built in bencher)
// ... before digging deeper we need to introduce some Rust concepts ...
// Slide 3: enums and match expressions
// Slide 4: traits - declaring, implementing, trait bounds, dyn trait
// Slide 5: Iterator trait and some examples
// Slide 6: why iterators are fast, part 1: monomorphisation (show some x86)
// Slide 7: why iterators are fast, part 2: LLVM (inlining, loop invariant motion, autovectorisation)
// Slide 8: why iterators are fast, part 3: try_fold (show more x86)
// Slide 9: why LINQ is slow, part 1: function pointers
// Slide 10: why LINQ is slow, part 2: no inlining, worse optimiser
// Slide 11: can we copy the Rust approach in C#?
// Bonus slide: data race in parallel iterator, brief digression about closures

//--------------------------------------
// Iterator performance
//--------------------------------------

// Most of the examples live in examples/, but this
// one goes here so that we can use cargo-asm (which
// doesn't yet support finding assembly from binaries)

// When comparing rust and .net, write a tool to use benchmarkdotnet for .net
// and standard bench harness plus cargo-asm for rust

pub fn sum_sq_iter(xs: &[f32]) -> f32 {
    xs.iter().map(|x| x * x).sum()
}

pub fn sum_sq_loop(xs: &[f32]) -> f32 {
    let mut sum_sq = 0.0;
    for x in xs {
        sum_sq += x * x;
    }
    sum_sq
}

pub fn dot_product_iter(xs: &[f32], ys: &[f32]) -> f32 {
    xs.iter().zip(ys).map(|(x, y)| x * y).sum()
}

pub fn dot_product_loop(xs: &[f32], ys: &[f32]) -> f32 {
    let mut result = 0f32;
    for i in 0..xs.len() {
        result += xs[i] * ys[i];
    }
    result
}

//--------------------------------------
// Iterator performance - bonus round
//--------------------------------------

enum ChainState { Front, Back }

pub struct NaiveChain<A, B> {
    a: A,
    b: B,
    state: ChainState
}

impl <A, B> NaiveChain<A, B> {
    pub fn new(a: A, b: B) -> Self {
        NaiveChain { a, b, state: ChainState::Front }
    }
}

impl<A, B> Iterator for NaiveChain<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>
{
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            ChainState::Front => match self.a.next() {
                elt @ Some(..) => elt,
                None => {
                    self.state = ChainState::Back;
                    self.b.next()
                }
            },
            ChainState::Back => self.b.next()
        }
    }
}

// Detecting data races at compile time: examples/data_race.rs
// Traits: examples/traits.rs
// Serde/procedural macros: examples/proc_macros.rs

// basic ownership and borrowing, standard C++ iterator invalidation error
// very quick mention of declarative and procedural macros. dbg macro
// playground, godbolt, cargo asm

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    fn data() -> Vec<f32> {
        let mut xs = vec![];
        for i in 0..1000 {
            xs.push(i as f32 / 17.4);
        }
        xs
    }

    // Could use the paste crate to make macro invocations
    // a bit nicer. Definitely not worth it for its own sake, but
    // might be worth discussing?

    macro_rules! bench_unary_array_function {
        ($bench_name:ident, $fn_name:ident) => {
            #[bench]
            fn $bench_name(b: &mut Bencher) {
                let xs = black_box(data());
                b.iter(|| {
                    black_box($fn_name(&xs));
                });
            }
        }
    }

    macro_rules! bench_binary_array_function {
        ($bench_name:ident, $fn_name:ident) => {
            #[bench]
            fn $bench_name(b: &mut Bencher) {
                let (xs, ys) = (black_box(data()), black_box(data()));
                b.iter(|| {
                    black_box($fn_name(&xs, &ys));
                });
            }
        }
    }

    bench_unary_array_function!(bench_sum_sq_loop, sum_sq_loop);
    bench_unary_array_function!(bench_sum_sq_iter, sum_sq_iter);

    bench_binary_array_function!(bench_dot_product_loop, dot_product_loop);
    bench_binary_array_function!(bench_dot_product_iter, dot_product_iter);

    #[bench]
    fn bench_naive_chain(b: &mut Bencher) {
        let xs = black_box(data());
        let ys = black_box(data());

        b.iter(|| {
            let zs = NaiveChain::new(xs.iter(), ys.iter());
            let s: f32 = zs.map(|x| x * x).sum();
            black_box(s);
        });
    }

    #[bench]
    fn bench_actual_chain(b: &mut Bencher) {
        let xs = black_box(data());
        let ys = black_box(data());

        b.iter(|| {
            let zs = xs.iter().chain(ys.iter());
            let s: f32 = zs.map(|x| x * x).sum();
            black_box(s);
        });
    }
}
