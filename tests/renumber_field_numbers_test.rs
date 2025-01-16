use std::io::Read;

#[test]
pub fn test() {
    proto_renumber::renumber_field_numbers("example.proto", "output.proto").unwrap();
    let mut f = std::fs::File::open("output.proto").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    assert_eq!(
        buffer,
        r#"syntax = "proto3";

message User {
  string name = 1;
  int32 age = 2;
  string email = 3;
}

message Product {
  int32 id = 1;
  string description = 2;
  double price = 3;
}
"#
    );
}
