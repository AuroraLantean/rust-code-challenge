/*
*/
struct Person<PetType: Animal> {
  //generic fields
  first_name: String,
  pet: PetType, //group type
}
trait Animal {
  fn make_sound(&self) -> (); //self, because it is a instance level method that we call on each instance of a type. E.g. not all dogs bark. some dogs bark.
}
struct Dog {}
impl Animal for Dog {
  //impl Animal for Dog... OR error: Trait bound not satisfied... must implement Animal trait for Dog type!

  //implement make_sound() OR error: not all trait items implemented
  fn make_sound(&self) -> () {
    println!("Dog barked!");
  }
}
struct Cat {}
impl Animal for Cat {
  fn make_sound(&self) -> () {
    println!("Cat meowed!");
  }
}

pub fn make_person() {
  let dog: Dog = Dog {}; //this type must implement Animal trait or satisfy Animal trait!
  let john = Person {
    first_name: "John".to_string(),
    pet: dog,
  };
  john.pet.make_sound();

  let cat: Cat = Cat {};
  let jane = Person {
    first_name: "Jane".to_string(),
    pet: cat,
  };
  jane.pet.make_sound();
}

#[test]
fn generics_traits() {
  make_person();
}
