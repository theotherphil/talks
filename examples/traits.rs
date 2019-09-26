
//--------------------------------------
// Some fundamental traits
//--------------------------------------

// What does equality mean for functions?
// How should YourCustomType be displayed in debug logging, or to the user?

// traits example - generics and trait objects
//      Debug, Hash, PartialOrd, Ord, PartialEq, Eq, etc.
//      don't just define everything by default on all objects with
//      a possibly nonsense implementation

trait Render {
    /// Not a great example, as there are already traits for this...
    fn render(&self) -> String;
}

struct Bees {
    rows: usize
}

impl Render for Bees {
    fn render(&self) -> String {
        std::iter::repeat("\u{1F41D}\n")
            .take(self.rows)
            .collect()
    }
}

struct Cactii {
    cols: usize
}

impl Render for Cactii {
    fn render(&self) -> String {
        std::iter::repeat("\u{1F335}")
            .take(self.cols)
            .collect()
    }
}

/// Static dispatch - monomorphised
fn write_to_console_static<R: Render>(xs: &[R]) {
    for x in xs {
        println!("{}", x.render());
    }
}

/// Dynamic dispatch
fn write_to_console_dynamic(xs: &[Box<dyn Render>]) {
    for x in xs {
        println!("{}", x.render());
    }
}

fn main() {
    let bees = vec![Bees { rows: 2}, Bees { rows: 3} ];
    write_to_console_static(&bees);

    let cactii = vec![Cactii { cols: 2}, Cactii { cols: 3} ];
    write_to_console_static(&cactii);

    let renderable: Vec<Box<dyn Render>> = vec![
        Box::new(Bees { rows: 2 }),
        Box::new(Cactii { cols: 3 })
    ];
    write_to_console_dynamic(&renderable);
}
