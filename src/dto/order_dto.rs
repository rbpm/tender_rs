use crate::traits::Data;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
//#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct OrderDto {
    pub object_id: String,
    pub title: String,
    // organizationId: Option<String>,
    // organizationName: Option<String>,
    // organizationPartName: Option<String>,
    // organizationCity: Option<String>,
    // organizationProvince: Option<String>,
    // bzpNumber: Option<String>,
    // tenderType: Option<String>,
    // competitionType: Option<String>,
    // concessionType: Option<String>,
    pub submission_offers_date: Option<String>,
    // tenderState: Option<String>,
    //#[serde(rename="isTenderAmountBelowEU")]
    pub is_tender_amount_below_e_u: bool,
    // tedContractNoticeNumber: Option<String>,
    pub initiation_date: String,
}

impl Data for OrderDto {
    fn name(&self) -> String {
        self.title.clone()
    }
    fn href(&self) -> String {
        "https://ezamowienia.gov.pl/mp-client/search/list/".to_owned() + &*self.object_id
    }
    fn date(&self) -> String {
        self.submission_offers_date
            .clone()
            .unwrap_or("null".to_owned())
    }
    fn id(&self) -> String {
        self.object_id.clone()
    }
}
