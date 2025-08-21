const NUMBER_OF_MONTHS: u32 = 12;
static SEASONS: [&str; 4] = ["Spring", "Summer", "Autumn", "Winter"];

fn print_months() {
    println!("Number of months in the year: {NUMBER_OF_MONTHS}");
}

fn main() {
    print_months();
}
