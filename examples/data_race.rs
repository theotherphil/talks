use std::sync::atomic::{AtomicU64, Ordering};
use rayon::prelude::*;

#[derive(Debug)]
struct Image {
    source: String
}

fn load_image(path: &str) -> Image {
    Image { source: path.into() }
}

// Examples:
// correct loading
// loading with invalid mutation
// fixing loading with invalid mutation
// more realistic example, e.g. global calculation engine that
// you accidentally call in a background thread

fn load_images_sequentially() {
    println!("load_images_sequentially");

    let paths = vec!["foo.png", "bar.png", "baz.png"];

    let images: Vec<Image> = paths
        .iter()
        .map(|x| load_image(x))
        .collect();

    println!("{:#?}", images);
}

fn load_images_in_parallel() {
    println!("load_images_in_parallel");

    let paths = vec!["foo.png", "bar.png", "baz.png"];

    let images: Vec<Image> = paths
        .par_iter()
        .map(|x| load_image(x))
        .collect();

    println!("{:#?}", images);
}

fn load_images_with_date_race() {
    println!("load_images_with_date_race");

    let paths = vec!["foo.png", "bar.png", "baz.png"];
    //let mut count = 0;
    let count = AtomicU64::new(0);

    let images: Vec<Image> = paths
        .par_iter()
        .map(|x| {
            //count += 1;
            count.fetch_add(1, Ordering::SeqCst);
            load_image(x)
        } )
        .collect();

    println!("{:#?}, {:?}", images, count);
}

fn main() {

    load_images_sequentially();
    load_images_in_parallel();
    load_images_with_date_race();
}
