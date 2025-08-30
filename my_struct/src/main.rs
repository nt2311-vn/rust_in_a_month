struct FileDirectory;
struct ColorRgb(u8, u8, u8);

fn main() {
    let my_color = ColorRgb(50, 0, 50);
    println!("The second part of the color is: {}", my_color.1);
}
