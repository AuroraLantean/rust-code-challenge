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

#[derive(Debug)]
struct PersonRef<'a> {
  name: &'a str,
  age: &'a i32,
}
impl<'a> Default for PersonRef<'a> {
  fn default() -> Self {
    Self {
      name: &"na",
      age: &0,
    }
  }
}

#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: i32,
}

fn a_must_outlive<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
  'b: 'a,
{
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
fn shortest_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

#[test]
fn testa5() {
  let movie = Movie::new("Xman", 9.3);
  println!("{:#?}", movie);

  let _person1 = PersonRef::default();

  let first_name = "Nouman";
  let nouman = Person {
    name: &first_name,
    age: 40,
  };
  println!("The name:{}, age: {}", nouman.name, nouman.age);

  let s_1 = "Helloooo";
  let s_2 = String::from("World");
  {
    let out1 = a_must_outlive(s_1, s_2.as_str());
    println!("{}", out1);

    let s_2b = String::from("World");
    let out2 = shortest_lifetime(s_1, s_2b.as_str());
    println!("{}", out2);
  }
  // println!("{}", v2);
}
