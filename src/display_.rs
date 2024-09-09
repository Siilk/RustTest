use std::fmt;
use std::fmt::Formatter;


#[allow(dead_code)]
fn run_display()
{
    struct DispStruct(i32);

    impl fmt::Display for DispStruct
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
        {
            return write!(f, "{}", self.0);
        }
    }

    let asdf: DispStruct = DispStruct(42);

    println!("{asdf}")
}