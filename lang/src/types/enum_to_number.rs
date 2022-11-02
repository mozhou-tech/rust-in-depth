#![allow(unused)]
enum MyEnum {
    A,
    B,
    C,
}
impl TryFrom<i32> for MyEnum {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == MyEnum::A as i32 => Ok(MyEnum::A),
            x if x == MyEnum::B as i32 => Ok(MyEnum::B),
            x if x == MyEnum::C as i32 => Ok(MyEnum::C),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enum_to_number() {
        let x = MyEnum::C as i32;
        println!("MyEnum::C in number : {:?}",x);
        match x.try_into() {
            Ok(MyEnum::A) => println!("a"),
            Ok(MyEnum::B) => println!("b"),
            Ok(MyEnum::C) => println!("c"),
            Err(_) => eprintln!("unknown number"),
        }
    }
}