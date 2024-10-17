/*
*/
struct Person<PetType: Animal + NotDangerous, Exotic: Animal + Dangerous> {
  //generic fields
  first_name: String,
  pet: PetType, //group of traits
  exotic: Exotic,
}
trait Animal {
  fn make_sound(&self) -> (); //self, because it is a instance level method that we call on each instance of a type. E.g. not all dogs bark. some dogs bark.
}
trait NotDangerous {}
trait Dangerous {
  fn bite(&self) -> ();
}
struct Dog {}
impl NotDangerous for Dog {}
impl Animal for Dog {
  //impl Animal for Dog... OR error: Trait bound not satisfied... must implement Animal trait for Dog type!
  //add `dyn` keyword before this trait.. OR ad "for Dog"

  //implement make_sound() OR error: not all trait items implemented
  fn make_sound(&self) -> () {
    println!("Dog barked!");
  }
}
struct Cat {}
impl NotDangerous for Cat {}
impl Animal for Cat {
  fn make_sound(&self) -> () {
    println!("Cat meowed!");
  }
}

struct Bear {}
impl Dangerous for Bear {
  fn bite(&self) -> () {
    println!("Bear bited!")
  }
}
impl Animal for Bear {
  fn make_sound(&self) -> () {
    println!("Bear roared!");
  }
}
struct Tiger {}
impl Dangerous for Tiger {
  fn bite(&self) -> () {
    println!("Tiger bited!");
  }
}
impl Animal for Tiger {
  fn make_sound(&self) -> () {
    println!("Tiger roared!");
  }
}

pub fn make_person() {
  let bear: Bear = Bear {};
  let tiger: Tiger = Tiger {};
  let dog: Dog = Dog {}; //this type must implement Animal trait or satisfy Animal trait!
  let john = Person {
    first_name: "John".to_string(),
    pet: dog,
    exotic: bear,
  };
  john.pet.make_sound();
  john.exotic.make_sound();
  john.exotic.bite();

  let cat: Cat = Cat {};
  let jane = Person {
    first_name: "Jane".to_string(),
    pet: cat,
    exotic: tiger,
  };
  jane.pet.make_sound();
  jane.exotic.make_sound();
  jane.exotic.bite();

  /*let vlad = Person {
    first_name: "Vlad".to_string(),
    pet: bear,
  };
  vlad.pet.make_sound();*/
}

#[test]
fn generics_traits() {
  make_person();
}
