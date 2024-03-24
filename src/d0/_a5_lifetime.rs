use std::fmt::Display;

#[derive(Debug)]
struct Movie<'a, T> {
    title: &'a str,
    rating: T,
} //Both of these need to have the same lifetime.

// 'a + Display + PartialOrd : The type T is a generic type that must implement both the Display and PartialOrd traits, AND type T must have a lifetime long enough to persist in the program.
// the Display trait ensures that the value being passed is printable, and the PartialOrd trait ensures that the type should be able to be compared partially using operators like <, >, >=, or<=
impl<'a, T: 'a + Display + PartialOrd> Movie<'a, T> {
    fn new(title: &'a str, rating: T) -> Self {
        Movie { title, rating }
    }
}

#[test]
fn testa5() {
    let movie = Movie::new("The Shawshank Redemption", 9.3);
    println!("{:#?}", movie);
}
