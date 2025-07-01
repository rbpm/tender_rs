use crate::dto::*;
use crate::traits::Data;
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
    process_data(
        &args,
        &order_data,
        IT_ORDERS,
        ORDERS,
        IT_ORDERS_FILENAME,
        ORDERS_FILENAME,
    );

    let tender_data = pages::get_tender_pages(&args, tender_old_all_data);
    println!("tender_data.len: {:?}", tender_data.len());

    process_data(
        &args,
        &order_data,
        IT_TENDERS,
        TENDERS,
        IT_TENDERS_FILENAME,
        TENDERS_FILENAME,
    );
}

fn process_data(
    args: &ArgDto,
    order_data: &Vec<Box<dyn Data>>,
    it_sheet_name: &str,
    sheet_name: &str,
    it_file_name: &str,
    file_name: &str,
) {
    let mut data_book = new_file_empty_worksheet();
    let mut all_data_book = new_file_empty_worksheet();
    data_book = process(data_book, &order_data, it_sheet_name, &args, false);
    all_data_book = process(all_data_book, &order_data, sheet_name, &args, true);
    let _ = writer::xlsx::write(&data_book, std::path::Path::new(it_file_name));
    let _ = writer::xlsx::write(&all_data_book, std::path::Path::new(file_name));
}
