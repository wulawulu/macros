use macros::AutoDebug;

#[allow(unused)]
#[derive(AutoDebug)]
pub struct RespBulkString {
    inner: String,
    #[debug(skip)]
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
