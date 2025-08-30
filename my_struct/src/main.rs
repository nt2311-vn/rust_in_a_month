struct FileDirectory;
struct ColorRgb(u8, u8, u8);
struct SizeAndColor {
    size: u32,
    color: ColorRgb,
}

struct Country {
    population: u32,
    capital: String,
    leader_name: String,
}

fn main() {
    let my_color = ColorRgb(50, 0, 50);
    println!("The second part of the color is: {}", my_color.1);

    let size_and_color = SizeAndColor {
        size: 150,
        color: my_color,
    };

    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");

    let kalmykia = Country {
        population,
        capital,
        leader_name,
    };
}
