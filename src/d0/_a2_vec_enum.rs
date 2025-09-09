/* */
pub struct User {
  name: String,
  balance: u64,
}

pub enum UserEvent {
  UserCreated(User),
  UserNameChanged { new_name: String },
  UserFundsAdded(u64),
  UserFundsSubtracted(u64),
}

fn main() {
  let user_events = vec![
    UserEvent::UserCreated(User {
      name: "Anton".to_string(),
      balance: 0,
    }),
    UserEvent::UserFundsAdded(12),
    UserEvent::UserFundsSubtracted(2),
  ];
  let users = find_users(user_events);
}
fn find_users(user_events: Vec<UserEvent>) -> Vec<User> {
  let mut output = vec![];
  for item in user_events {
    match item {
      UserEvent::UserCreated(user) => output.push(user),
      _ => (),
    };
  }
  return output;
}
