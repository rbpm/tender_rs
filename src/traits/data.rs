
pub trait Data {
    fn name(&self) -> String;
    fn href(&self) -> String;
    fn date(&self) -> String;
    fn id(&self) -> String;

    fn is_i_t(&self) -> bool {
        is_i_t(&*self.name())
    }
}

pub fn is_i_t(name: &str) -> bool {
    let lower_name = name.to_lowercase();
    lower_name.contains("oprogramowani")
        || lower_name.contains(" it ")
        || lower_name.contains("rozwój i utrzymanie systemu")
    || (lower_name.contains("aplikacj") && !lower_name.contains("folii"))
}

pub fn is_in_vec(tenders: &Vec<Box<dyn Data>>, tender: Box<&dyn Data>) -> bool {
    for p in tenders {
        if p.id() == tender.id() && p.name() == tender.name() {
            println!("contain: {}:{}", p.id(), p.name());
            return true;
        }
    }
    return false;
}
