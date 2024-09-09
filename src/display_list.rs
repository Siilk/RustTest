#[allow(dead_code)]
#[allow(unused)]
#[allow(non_snake_case)]
fn run_display_list()
{
    struct List(Vec<i32>);

    impl fmt::Display for List
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            let vec = &self.0;

            write!(f, "[")?;
            for (i, v) in vec.iter().enumerate()
            {
                if i != 0
                {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v);
            }
            return write!(f, "]");
        }
    }

    let disV = List(vec!(1, 2, 3));
    println!("{disV}");
}