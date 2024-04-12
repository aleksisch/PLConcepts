use linq::fib::Fib;
use linq::linq_impl::LinqExt;

fn main() {
    let x = Fib::new();

    for i in x.my_where(|&val| val % 3 == 0)
                    .select(|val| if val % 2 == 0 {val * val} else {val})
                    .my_take(5) {
        print!("{} ", i);
    }
}