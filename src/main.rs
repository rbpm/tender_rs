use crate::dto::*;
use crate::xlsx::*;
use clap::Parser;
use umya_spreadsheet::*;

mod dto;
mod pages;
mod traits;
mod xlsx;

fn main() {
    let args = ArgDto::parse();
    println!("ArgDto: {:?}", args);

    let tender_old_all_data = get_old_all_data(TENDERS_FILENAME, TENDERS);
    let order_old_all_data = get_old_all_data(ORDERS_FILENAME, ORDERS);
    println!("Tender old data len: {:?}", tender_old_all_data.len());
    println!("Order old data len: {:?}", order_old_all_data.len());
    
    
    let order_data = pages::get_order_pages(&args, order_old_all_data);
    println!("order_data.len: {:?}", order_data.len());

    let mut order_book = new_file_empty_worksheet();
    let mut all_order_book = new_file_empty_worksheet();
    order_book = process(order_book, &order_data, IT_ORDERS, &args, false);
    all_order_book = process(all_order_book, &order_data, ORDERS, &args, true);
    let _ = writer::xlsx::write(&order_book, std::path::Path::new(IT_ORDERS_FILENAME));
    let _ = writer::xlsx::write(&all_order_book, std::path::Path::new(ORDERS_FILENAME));

    let tender_data = pages::get_tender_pages(&args, tender_old_all_data);
    println!("tender_data.len: {:?}", tender_data.len());

    let mut tender_book = new_file_empty_worksheet();
    let mut all_tender_book = new_file_empty_worksheet();
    tender_book = process(tender_book, &tender_data, IT_TENDERS, &args, false);
    all_tender_book = process(all_tender_book, &tender_data, TENDERS, &args, true);
    let _ = writer::xlsx::write(&tender_book, std::path::Path::new(IT_TENDERS_FILENAME));
    let _ = writer::xlsx::write(&all_tender_book, std::path::Path::new(TENDERS_FILENAME));
}
