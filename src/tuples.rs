#[allow(dead_code)]
#[allow(unused)]
#[allow(non_snake_case)]
fn run_tuples()
{
    fn reverse(pair: (i32, bool)) -> (bool, i32)
    {
        let (intVal, boolVal) = pair;

        return (boolVal, intVal);
    }


    println!("{:?}", reverse((42, true)));

    let loongcat = (1, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, "n", 9);

    //print!("{loongcat}");

}