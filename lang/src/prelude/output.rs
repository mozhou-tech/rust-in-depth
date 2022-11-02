
fn print_hello(){
    println!("Hello {:}","World");
}

fn output_panic(){
    panic!("Hello i'm panic {:}.","dog")
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_print_hello(){
        print_hello()
    }

    #[test]
    fn test_output_panic(){
        output_panic()
    }

}