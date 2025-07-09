use crate::dto::{ArgDto, DataDto};
use crate::traits::{is_in_vec, Data};
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use std::io::Cursor;

pub fn get_tender_pages(args: &ArgDto, old_all: Vec<Box<dyn Data>>) -> Vec<Box<dyn Data>> {
    let mut data_vec = Vec::<Box<dyn Data>>::new();
    for tender_page in 1..args.tender_pages + 1 {
        //sleep(Duration::from_secs(1));
        println!("tender page: {}", tender_page);
        let (data_vec_1, done) = get_tender_page(data_vec, tender_page, &old_all);
        if done {
            return data_vec_1;
        }
        data_vec = data_vec_1;
    }
    data_vec
}
pub fn get_tender_page(
    mut data_vec: Vec<Box<dyn Data>>,
    tender_page: u32,
    old_all: &Vec<Box<dyn Data>>,
) -> (Vec<Box<dyn Data>>, bool) {
    let tender_prefix = "https://oneplace.marketplanet.pl/zapytania-ofertowe-przetargi/-/rfp/cat?_7_WAR_organizationnoticeportlet_cur=";
    let tender_url = format!("{}{}", tender_prefix, tender_page);
    //reqwest::blocking::get is breaking tender server responses
    // let client = Client::builder()
    //     .timeout(Duration::from_secs(60 * 30)) // Total request timeout
    //     .connect_timeout(Duration::from_secs(60 * 5)) // Connection establishment timeout
    //     .build()
    //     .unwrap();
    // let tender_response = client.get(tender_url).send().unwrap();
    // if let Ok(tender_response) = reqwest::blocking::get(tender_url) {
    //     if let Ok(html) = tender_response.text() {
    if let Ok(mut tender_response) = ureq::get(tender_url).call() {
        if let Ok(html) = tender_response.body_mut().read_to_string() {
            let cursor = Cursor::new(html);
            let document = Document::from_read(cursor).unwrap();
            for node in document.find(Attr(
                "id",
                "_7_WAR_organizationnoticeportlet_selectNoticesSearchContainer",
            )) {
                for sub_node in node.find(Class("lfr-search-container-list")) {
                    for group in sub_node.find(Name("dl")) {
                        for element in group.find(Attr("data-qa-id", "row")) {
                            for a_tag in element.find(Name("a")) {
                                let href_value = a_tag.attr("href").unwrap();
                                let name_value = a_tag.text();
                                for date_span in
                                    element.find(Attr("title", "Termin składania ofert"))
                                {
                                    let date_value = date_span.text().trim().to_string();
                                    let data_dto = DataDto {
                                        name: name_value.to_string(),
                                        href: href_value.to_string(),
                                        date: date_value.to_string(),
                                        id: get_href_id(href_value).to_string(),
                                    };
                                    if is_in_vec(old_all, Box::new(&data_dto)) {
                                        data_vec.push(Box::new(data_dto));
                                        return (data_vec, true);
                                    }
                                    //data_vec.extend(std::iter::once(Box::new(data_dto)));
                                    data_vec.push(Box::new(data_dto));
                                }
                            }
                        }
                    }
                }
            }
        } else {
            println!("tender page could not be read");
            return (data_vec, true);
        }
    } else {
        println!("tender page is not reachable");
        return (data_vec, true);
    }
    (data_vec, false)
}
fn get_href_id(value: &str) -> &str {
    if value.len() < 10 {
        return &"len err";
    }
    if let Some(pos) = value.find("_noticeId=") {
        let id = &value[pos + 10..];
        return id;
    }
    &"index err"
}
