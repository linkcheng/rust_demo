fn main() {
    println!("Hello, world!");
    greet_world();

    let r= &1;
    let &a= r;
    let ref a= r;
    let a= &r;
    let a= *r;
    println!("r={:?} a={:?}", r, a)
}

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "Hello world";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", region)
    }

    let x = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        .iter()
        .map(|x| x + 1)
        .fold(0, |x, y| x + y);
    println!("{}", x);

    let pair = ('a', 17);
    println!("{}", pair.0);

    println!("{}", real());

    let i = 1;
    let exe = |x| x + i;
    println!("{}", exe(5));

    let v1 = vec![1, 2, 3];
    println!("{:?}", v1);

    let is_v1_equal = move |x| x == v1;
    let v2 = vec![1, 2, 3];
    println!("{}", is_v1_equal(v2));
    let v3 = vec![1, 2, 4];
    println!("{}", is_v1_equal(v3));
}

fn real() -> i32 {
    return 5;
}
