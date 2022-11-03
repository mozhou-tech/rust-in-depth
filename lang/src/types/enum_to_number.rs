#![allow(unused)]

use num_enum::IntoPrimitive;

#[repr(i32)]
enum MyEnum {
    A,
    B,
    C,
}

#[derive(IntoPrimitive)]
#[repr(u8)]
enum Number {
    Zero,
    One,
    Two,
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
        println!("MyEnum::C in number : {:?}", x);
        match x.try_into() {
            Ok(MyEnum::A) => println!("a"),
            Ok(MyEnum::B) => println!("b"),
            Ok(MyEnum::C) => println!("c"),
            Err(_) => eprintln!("unknown number"),
        }
    }

    #[test]
    fn test_enum_to_number_transmute() {
        let x = MyEnum::C;
        let y = x as i32;
        let z: MyEnum = unsafe { std::mem::transmute(y) };

        // match the enum that came from an int
        match z {
            MyEnum::A => { println!("Found A"); }
            MyEnum::B => { println!("Found B"); }
            MyEnum::C => { println!("Found C"); }
        }
    }

    /// 使用 num_enum 库
    #[test]
    fn test_num_enum_lib() {
        let x: u8 = Number::Two.into();
        println!("{:?}", x)
    }
}