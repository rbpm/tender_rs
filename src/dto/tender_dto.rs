use crate::traits::Data;

#[derive(Debug, Clone)]
pub struct TenderDto {
    pub name: String,
    pub href: String,
    pub date: String,
    pub id: String,
}

impl Data for TenderDto {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn href(&self) -> String {
        self.href.clone()
    }
    fn date(&self) -> String {
        self.date.clone()
    }
    fn id(&self) -> String {
        self.id.clone()
    }
}

pub fn new_tender_dto(name: &str, href: &str, date: &str, id: &str) -> TenderDto {
    TenderDto {
        name: name.to_string(),
        href: href.to_string(),
        date: date.to_string(),
        id: id.to_string(),
    }
}
