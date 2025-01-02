use macros::AutoDeref;

#[allow(unused)]
#[derive(Debug, AutoDeref)]
#[deref(field = "inner", mutable = "true")]
pub struct RespBulkString {
    inner: String,
    nothings: (),
    hello: u32,
}

fn main() {
    let resp = RespBulkString {
        inner: "hello".to_string(),
        nothings: (),
        hello: 0,
    };
    println!("{:?}", resp);
}
