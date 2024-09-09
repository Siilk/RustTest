fn main()
{
    let int: i32 = 6969;

    let real: f64 = 3.1415926;

    println!("{real:.3}");


    #[derive(Debug)]
    struct Wakka(i32);

    let a:Wakka = Wakka(42);

    #[derive(Debug)]
    struct WakkaWakka(Wakka);

    let b:WakkaWakka = WakkaWakka(Wakka(69));


    println!("{a:?}\n");
    println!("{a:#?}");

    println!("{b:?}\n");
    println!("{b:#?}");

}