use clap::Parser;

pub const TENDERS_FILENAME: &'static str = "przetargi.xlsx";
pub const TENDERS: &'static str = "przetargi";
pub const ORDERS_FILENAME: &'static str = "oferty.xlsx";
pub const ORDERS: &'static str = "oferty";
pub const IT_TENDERS: &'static str = "przetargi IT";
pub const IT_ORDERS: &'static str = "oferty IT";
pub const IT_TENDERS_FILENAME: &'static str = "przetargi_it.xlsx";
pub const IT_ORDERS_FILENAME: &'static str = "oferty_it.xlsx";

/// Simple program to get tenders
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ArgDto {
    /// Save all tenders to excel
    #[arg(long, default_value_t = true)]
    pub save_all: bool,
    /// Number of tender Pages to get (max 1000)
    #[arg(long, default_value_t = 1000)]
    pub tender_pages: u32,
    /// Number of order Pages to get (max 1000/50=200)
    #[arg(long, default_value_t = 200)]
    pub order_pages: u32,
    /// Append old tenders to new all
    #[arg(short, long, default_value_t = false)]
    pub append_all: bool,
    /// Tender old file name
    #[arg(long, default_value = TENDERS_FILENAME)]
    pub tender_old_filename: String,
    /// Order old file name
    #[arg(long, default_value = ORDERS_FILENAME)]
    pub orders_old_filename: String,
}