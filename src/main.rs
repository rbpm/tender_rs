use crate::dto::*;
use crate::xlsx::*;
use chrono::offset::Local;
use clap::Parser;

mod dto;
mod pages;
mod traits;
mod xlsx;

fn file_date_str() -> String {
    let date_time = Local::now();
    format!("{}", date_time.format("%Y%m%d"))
}

fn main() {
    let args = ArgDto::parse();
    println!("ArgDto: {:?}", args);

    let tender_old_all_data = get_old_all_data(TENDERS_FILENAME, TENDERS);
    let order_old_all_data = get_old_all_data(ORDERS_FILENAME, ORDERS);
    println!("Tender old data len: {:?}", tender_old_all_data.len());
    println!("Order old data len: {:?}", order_old_all_data.len());

    let order_data = pages::get_order_pages(&args, order_old_all_data);
    println!("order_data.len: {:?}", order_data.len());
    process_data(
        &args,
        &order_data,
        IT_ORDERS,
        ORDERS,
        &format!("{}_{}.xlsx", IT_ORDERS_FILENAME, file_date_str()),
        ORDERS_FILENAME,
    );

    let tender_data = pages::get_tender_pages(&args, tender_old_all_data);
    println!("tender_data.len: {:?}", tender_data.len());

    process_data(
        &args,
        &order_data,
        IT_TENDERS,
        TENDERS,
        &format!("{}_{}.xlsx", IT_TENDERS_FILENAME, file_date_str()),
        TENDERS_FILENAME,
    );
}
