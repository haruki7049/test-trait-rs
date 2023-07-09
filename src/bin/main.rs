use test_trait_rs;

fn main() {
    let hoge = test_trait_rs::Haruki::new("haruki".to_string(), 17);

    println!("{:?}", hoge);
}
