use std::thread::sleep;

fn expensive_calc(_n: &i32) {
    sleep(std::time::Duration::from_secs(1))
}

fn main() {
    let v = vec![1, 2, 3];
    dbg!(&v);
    use tqrs::TqrsIteratorExt;
    for n in v
        .iter()
        .tqrs()
        .with_bound()
        .with_delims(('<', '>'))
        .with_bar("*")
    {
        expensive_calc(n);
    }
    for n in (0..).tqrs() {
        expensive_calc(&n);
        if n == 10 {
            break;
        }
    }
    println!();
}
