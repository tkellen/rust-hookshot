extern crate hookshot;

fn main() {
    let result = hookshot::config::parse("\
      [group]\n\
        [[group.notifiers]]\n\
          url = \"http://second.com\"\n\
        [[group.notifiers]]\n\
          url = \"http://first.com\"\n\
    ");
    println!("{:?}", result);
}
