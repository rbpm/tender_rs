use crate::dto::{new_tender_dto, ArgDto};
use crate::traits::Data;
use umya_spreadsheet::{
    new_file_empty_worksheet, reader, writer, Cell, Color, Hyperlink, Spreadsheet, Worksheet,
};

pub fn get_old_all_data(file_name: &str, sheet_name: &str) -> Vec<Box<dyn Data>> {
    let path = std::path::Path::new(file_name);
    let mut order_vec = Vec::<Box<dyn Data>>::new();
    if let Ok(book) = reader::xlsx::read(path) {
        if let Some(sheet) = book.get_sheet_by_name(sheet_name) {
            for row in 2..sheet.get_highest_row() + 1 {
                let mut tender = new_tender_dto("", "", "", "");
                if let Some(id_cell) = sheet.get_cell((2, row)) {
                    tender.id = id_cell.get_value().to_string();
                }
                if let Some(date_cell) = sheet.get_cell((3, row)) {
                    tender.date = date_cell.get_value().to_string();
                }
                if let Some(href_cell) = sheet.get_cell((4, row)) {
                    tender.href = href_cell.get_value().to_string();
                }
                if let Some(name_cell) = sheet.get_cell((5, row)) {
                    tender.name = name_cell.get_value().to_string();
                }
                order_vec.push(Box::new(tender));
            }
        }
    }
    order_vec
}
pub fn process_data(
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
fn process(
    mut book: Spreadsheet,
    data_vec: &Vec<Box<dyn Data>>,
    sheet_name: &str,
    _args: &ArgDto,
    all: bool,
) -> Spreadsheet {
    let sheet = book.new_sheet(sheet_name).unwrap();
    set_header(all, sheet);
    let mut row = 1;
    for data in data_vec {
        if all || data.is_i_t() {
            row += 1;
            save_row_data(all, sheet, row, &data);
        }
    }
    book
}
fn set_header(all: bool, sheet: &mut Worksheet) {
    let row = 1;
    let mut col = 0;
    if all {
        col += 1;
        let it_cell = sheet.get_cell_mut((col, row));
        it_cell.set_value("IT");
        col += 1;
        let id_cell = sheet.get_cell_mut((col, row));
        id_cell.set_value("id");
    }
    col += 1;
    let data_cell = sheet.get_cell_mut((col, row));
    data_cell.set_value("data");
    col += 1;
    let href_cell = sheet.get_cell_mut((col, row));
    href_cell.set_value("link");
    col += 1;
    let name_cell = sheet.get_cell_mut((col, row));
    name_cell.set_value("nazwa");
}

fn save_row_data(all: bool, sheet: &mut Worksheet, row: u32, data: &&Box<dyn Data>) {
    let mut col = 0;
    if all {
        col += 1;
        let it_cell = sheet.get_cell_mut((col, row));
        if data.is_i_t() {
            it_cell.set_value("IT");
        }
        col += 1;
        let id_cell = sheet.get_cell_mut((col, row));
        id_cell.set_value(&data.id());
    }
    col += 1;
    let data_cell = sheet.get_cell_mut((col, row));
    data_cell.set_value(&data.date());
    col += 1;
    let href_cell = sheet.get_cell_mut((col, row));
    set_href(data.href(), href_cell);
    col += 1;
    let name_cell = sheet.get_cell_mut((col, row));
    name_cell.set_value(&data.name());
}

fn set_href(href: String, href_cell: &mut Cell) {
    let mut hyperlink1 = Hyperlink::default();
    hyperlink1.set_location(false);
    hyperlink1.set_url(&href);
    hyperlink1.set_tooltip("link");
    href_cell.set_hyperlink(hyperlink1);
    href_cell.set_value(&href);

    let style = href_cell.get_style_mut();
    let font = style.get_font_mut();
    let _ = font.set_underline("single");
    let mut color = Color::default();
    color.set_argb(Color::COLOR_BLUE);
    let _ = font.set_color(color);
}
