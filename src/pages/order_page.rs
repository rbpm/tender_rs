use crate::dto::OrderDto;
use crate::traits::{is_in_vec, Data};

pub fn get_order_pages(old_all: Vec<Box<dyn Data>>) -> Vec<Box<dyn Data>> {
    let mut order_vec = Vec::<Box<dyn Data>>::new();
    for order_page in 1..3 {
        println!("order page: {}", order_page);
        let (order_vec_1, done) = get_order_page(order_vec, order_page, &old_all);
        if done {
            return order_vec_1;
        }
        order_vec = order_vec_1;
    }
    order_vec
}

pub fn get_order_page(mut order_vec: Vec<Box<dyn Data>>, order_page: i32, old_all: &Vec<Box<dyn Data>>) -> (Vec<Box<dyn Data>>, bool) {
    let prefix = "https://ezamowienia.gov.pl/mp-readmodels/api/Search/SearchTenders?SortingColumnName=InitiationDate&SortingDirection=DESC&PageNumber=";
    let suffix = "&PageSize=50";
    let url = format!("{}{}{}", prefix, order_page, suffix);
    let response = reqwest::blocking::get(url).unwrap();
    let order_page_vec: Vec<OrderDto> = response.json().unwrap();
    for data_dto in order_page_vec {
        if is_in_vec(old_all, Box::new(&data_dto)){
            order_vec.push(Box::new(data_dto));
            return (order_vec, true);
        }
        order_vec.push(Box::new(data_dto));
    }
    return (order_vec, false);
}
