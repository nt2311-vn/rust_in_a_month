fn main() {
    let my_variable: i32;

    {
        let calculation_result = { 57 };
        my_variable = calculation_result;
        println!("{my_variable}");
    }
}
