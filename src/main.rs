fn main() {
    let s = String::from("hello");
    let f = move || s;
    let a = f();
    let b = f();
}
