fn main() 
{
    let x = fib(7);
    print!("{x}\t");
    let y = fib2(7);
    println!("{}\n", y);
}

fn fib(x : i32) -> i32
{
    if x==1 ||x==2
    {
        1
    }
    else
    {
        fib(x-2) + fib(x-1)
    }
}

fn fib2(x : i32) -> i32
{
    if x==1 || x==2 {1} else {fib2(x-2)+fib2(x-1)}
}
