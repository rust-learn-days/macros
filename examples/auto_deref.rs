use macros::DerefInfo;

#[allow(unused)]
#[derive(Debug, DerefInfo)]
#[deref(field = "inner")]
pub struct RespBulkString {
    inner: String,
    // nothing: (),
}

fn main() {
    let s = RespBulkString {
        inner: "hello".to_string(),
        // nothing: (),
    };
    println!("{:?}", s);
}
