use crate::traits::Data;

#[derive(Debug)]
pub struct DataDto {
    pub name: String,
    pub href: String,
    pub date: String, 
    pub id: String,
}

impl Data for DataDto {
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