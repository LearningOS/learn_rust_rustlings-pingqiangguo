//move_semantics6.rs
//让我编译！用于提示的“窸窸窣窣提示 move_semantics6”
//除了添加或删除引用之外，您无法更改任何内容


fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
