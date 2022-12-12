use serde::{Deserialize, Serialize};

use super::cargo::NovaServiceType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub warehouse_number: Option<String>,
    pub address_name: Option<String>,
    pub address_house: Option<String>,
    pub address_flat: Option<String>,
    pub pochtomat_number: Option<String>,
    pub service_type: NovaServiceType,
}

impl Address {
    pub fn warehouse(warehouse_number: i32) -> Self {
        Address {
            warehouse_number: Some(warehouse_number.to_string()),
            address_name: None,
            address_house: None,
            address_flat: None,
            pochtomat_number: None,
            service_type: NovaServiceType::WarehouseWarehouse,
        }
    }
    pub fn address(address_name: String, address_house: String, apartment_number: String) -> Self {
        Address {
            warehouse_number: None,
            address_name: Some(address_name),
            address_house: Some(address_house),
            address_flat: Some(apartment_number),
            pochtomat_number: None,
            service_type: NovaServiceType::WarehouseDoors,
        }
    }
    pub fn pochtomat(pochtomat_number: i32) -> Self {
        Address {
            warehouse_number: None,
            address_name: None,
            address_house: None,
            address_flat: None,
            pochtomat_number: Some(pochtomat_number.to_string()),
            service_type: NovaServiceType::WarehouseWarehouse,
        }
    }
}

pub struct Recipient {
    pub city_name: String,
    pub full_name: String,
    pub phone: String,
    pub is_payer: bool,
    pub address: Address,
}

impl Recipient {
    pub fn new(
        city: String,
        full_name: String,
        phone: &str,
        is_payer: bool,
        address: Address,
    ) -> Self {
        Recipient {
            city_name: city.to_owned(),
            full_name,
            phone: phone.to_owned(),
            is_payer,
            address,
        }
    }
}
