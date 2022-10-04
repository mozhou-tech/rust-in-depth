fn dont_assign_to_a_borrowed_value() {
    let mut a = 1;
    let ptr: &mut i32 = &mut a;
    let mut moved_a = a;
    moved_a = 2;
    println!("{}", ptr);
}

#[cfg(test)]
mod tests {
    use crate::dont_assign_to_a_borrowed_value::dont_assign_to_a_borrowed_value;

    #[test]
    fn test_dont_assign_to_a_borrowed_value() {
        dont_assign_to_a_borrowed_value();
    }
}