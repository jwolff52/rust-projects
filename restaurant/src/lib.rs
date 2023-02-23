mod front_of_house;

mod back_of_house;

use crate::front_of_house::{hosting, serving};

pub fn eat_at_restaurant() {
    hosting::add_to_wait_list();
    hosting::seat_at_table();
    serving::take_order();
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast with {} please!", meal.toast, meal.seasonal_fruit());

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("{:?}", order1);
    println!("{:?}", order2);

    serving::serve_order();
    serving::take_payment();
}

#[cfg(test)]
mod tests {
    use super::front_of_house::hosting;

    #[test]
    fn it_works() {
        let result = hosting::add_to_wait_list();
        assert_eq!(result, ());

        let result = hosting::seat_at_table();
        assert_eq!(result, ());
    }
}
