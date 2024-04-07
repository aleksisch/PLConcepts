use linq::fib;
use linq::fib::Fib;
use linq::linq_impl::LinqExt;

fn main() {
    let x = Fib::new();
    for i in x.select(|val| val * 2)
                    .my_where(|&val| val > 10).my_take(5) {
        print!("{} ", i);
    }
}