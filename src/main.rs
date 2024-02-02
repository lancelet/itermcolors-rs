use itermcolors_rs::ColorScheme;

fn main() {
    println!("Kitty color scheme:");

    let cs =
        ColorScheme::from_file("iceberg.itermcolors").expect("Could not parse iceberg.itermcolors");
    let kitty = cs.to_kitty();
    println!("{}", kitty);
}
