#[macro_export]
macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
}

#[allow(clippy::vec_init_then_push)]
fn main() {
    let v = my_vec!(1, 2, 3); //Vec<i32>
    println!("{:?}", v);
}
