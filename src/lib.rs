use std::fmt;

use properties::{NovaOptionsSeat, NovaRequestError};
use raw::NovaposhtaRaw;

use serde::{Deserialize, Serialize};
pub mod properties;
pub mod raw;

pub struct Novaposhta {
    pub raw: NovaposhtaRaw,
}

impl Novaposhta {
    pub fn new(api_key: String) -> Self {
        let raw = NovaposhtaRaw::new(api_key);
        Novaposhta { raw }
    }
}

impl Default for Novaposhta {
    fn default() -> Self {
        let raw = NovaposhtaRaw::default();
        Novaposhta { raw }
    }
}

impl Novaposhta {
    pub async fn create_new_ttn(
        &self,
        payload: CreateNewTtnPayload,
    ) -> Result<CreateNewTtnResponse, Box<dyn std::error::Error>> {
        let payer_type = if payload.recipient.is_payer {
            "Recipient".to_string()
        } else {
            "Sender".to_string()
        };
        let cargo_props = payload.get_props();
        let sender_city = self
            .get_city_ref(payload.sender.city.clone())
            .await
            .unwrap();
        let sender_props = self.get_first_sender().await?;
        let sender_address = self
            .get_warehouses_ref(
                payload.sender.warehouse_number.clone(),
                payload.sender.city.clone(),
            )
            .await
            .unwrap();
        if payload.recipient.address.warehouse_number.is_some() {
            let service_type = NovaServiceType::WarehouseWarehouse;
            let response = self
                .raw
                .internet_document_create_warehouse(
                    payer_type,
                    payload.payment_method.to_string(),
                    payload.cargo_type,
                    cargo_props.1,
                    service_type.to_string(),
                    cargo_props.0,
                    payload.description,
                    cargo_props.2,
                    sender_city,
                    sender_props.0,
                    sender_address,
                    sender_props.1,
                    payload.sender.phone,
                    payload.recipient.city,
                    payload.recipient.address.warehouse_number,
                    format!(
                        "{} {}",
                        payload.recipient.first_name, payload.recipient.last_name
                    ),
                    payload.recipient.phone,
                    payload.date_time,
                    None,
                    None,
                    None,
                    None,
                    None,
                )
                .await?;
            let data = response.data().unwrap();
            return Ok(CreateNewTtnResponse {
                ttn: data.IntDocNumber.unwrap(),
                ttn_ref: data.Ref.unwrap(),
                cost_delivery: data.CostOnSite.unwrap(),
                estimated_delivery_date: data.EstimatedDeliveryDate.unwrap(),
            });
        } else if payload.recipient.address.pochtomat_number.is_some() {
            let service_type = NovaServiceType::WarehouseWarehouse;
            let options = payload.get_seats_options();
            let pochtomat_ref = self
                .get_warehouses_ref(
                    payload.recipient.address.pochtomat_number.unwrap(),
                    payload.sender.city,
                )
                .await?;
            let response = self
                .raw
                .internet_document_create_warehouse(
                    payer_type,
                    payload.payment_method.to_string(),
                    payload.cargo_type,
                    cargo_props.1,
                    service_type.to_string(),
                    cargo_props.0,
                    payload.description,
                    cargo_props.2,
                    sender_city,
                    sender_props.0,
                    sender_address,
                    sender_props.1,
                    payload.sender.phone,
                    payload.recipient.city,
                    None,
                    format!(
                        "{} {}",
                        payload.recipient.first_name, payload.recipient.last_name
                    ),
                    payload.recipient.phone,
                    payload.date_time,
                    Some(options),
                    Some(pochtomat_ref),
                    None,
                    None,
                    None,
                )
                .await?;
            let data = response.data().unwrap();
            return Ok(CreateNewTtnResponse {
                ttn: data.IntDocNumber.unwrap(),
                ttn_ref: data.Ref.unwrap(),
                cost_delivery: data.CostOnSite.unwrap(),
                estimated_delivery_date: data.EstimatedDeliveryDate.unwrap(),
            });
        } else if payload.recipient.address.address_name.is_some() {
            let service_type = NovaServiceType::WarehouseWarehouse;
            let response = self
                .raw
                .internet_document_create_warehouse(
                    payer_type,
                    payload.payment_method.to_string(),
                    payload.cargo_type,
                    cargo_props.1,
                    service_type.to_string(),
                    cargo_props.0,
                    payload.description,
                    cargo_props.2,
                    sender_city,
                    sender_props.0,
                    sender_address,
                    sender_props.1,
                    payload.sender.phone,
                    payload.recipient.city,
                    None,
                    format!(
                        "{} {}",
                        payload.recipient.first_name, payload.recipient.last_name
                    ),
                    payload.recipient.phone,
                    payload.date_time,
                    None,
                    None,
                    Some(payload.recipient.address.address_name.unwrap()),
                    Some(payload.recipient.address.address_house.unwrap()),
                    Some(payload.recipient.address.address_flat.unwrap()),
                )
                .await?;
            let data = response.data().unwrap();
            return Ok(CreateNewTtnResponse {
                ttn: data.IntDocNumber.unwrap(),
                ttn_ref: data.Ref.unwrap(),
                cost_delivery: data.CostOnSite.unwrap(),
                estimated_delivery_date: data.EstimatedDeliveryDate.unwrap(),
            });
        } else {
            Err(Box::new(NovaRequestError::new(
                "You must specify address.".to_string(),
            )))
        }
    }

    async fn get_first_sender(&self) -> Result<(String, String), Box<dyn std::error::Error>> {
        let response = self
            .raw
            .counterparty_search(None, "Sender".to_string())
            .await?;
        let data = response.data().unwrap();
        let contact_couterparty = self
            .raw
            .get_counterparty_contact_persons(data.Ref.clone().unwrap())
            .await?
            .data()
            .unwrap()
            .Ref
            .unwrap();
        Ok((data.Ref.unwrap(), contact_couterparty))
    }

    async fn get_city_ref(&self, city: String) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.raw.get_cities(Some(city.clone())).await?;
        if response.success {
            let data = response.data().unwrap();
            return Ok(data.Ref.unwrap());
        }
        Err(Box::new(NovaRequestError::new(
            "Novaposhta response not success".to_string(),
        )))
    }

    async fn get_warehouses_ref(
        &self,
        warehouse_number: String,
        city: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.raw.get_warehouses(None, Some(city.clone())).await?;
        if response.success {
            let data = response.data.clone();
            for warehouse in data.iter() {
                let number_warehouse = warehouse.Number.clone().unwrap();
                if number_warehouse == warehouse_number {
                    return Ok(warehouse.Ref.clone().unwrap());
                }
            }
            return Err(Box::new(NovaRequestError::new(
                format!(
                    "Warehouse with number {} not found in {}",
                    warehouse_number,
                    city.clone()
                )
                .to_string(),
            )));
        }
        Err(Box::new(NovaRequestError::new(
            "Novaposhta response not success".to_string(),
        )))
    }
}

pub struct CreateNewTtnPayload {
    recipient: Recipient,
    sender: Sender,
    date_time: String,
    payment_method: NovaPaymentMethod,
    description: String,
    cargos: Vec<Cargo>,
    cargo_type: String,
}

pub struct Sender {
    city: String,
    warehouse_number: String,
    phone: String,
}

#[derive(Debug, Clone)]
pub struct CreateNewTtnResponse {
    pub ttn: String,
    pub ttn_ref: String,
    pub cost_delivery: i32,
    pub estimated_delivery_date: String,
}

impl CreateNewTtnPayload {
    fn get_props(&self) -> (i32, f32, i32) {
        let mut seats = 0;
        let mut weight: f32 = 0.0;
        let mut total_cost = 0;
        for cargo in self.cargos.iter() {
            seats += 1;
            weight += cargo.weight;
            total_cost += cargo.cost;
        }
        (seats, weight, total_cost)
    }
    fn get_seats_options(&self) -> Vec<NovaOptionsSeat> {
        let mut options: Vec<NovaOptionsSeat> = vec![];
        for cargo in self.cargos.iter() {
            options.push(cargo.options_seat.clone().unwrap());
        }
        options
    }
}

pub struct Recipient {
    city: String,
    first_name: String,
    last_name: String,
    phone: String,
    is_payer: bool,
    address: Address,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Address {
    pub warehouse_number: Option<String>,
    pub address_name: Option<String>,
    pub address_house: Option<String>,
    pub address_flat: Option<String>,
    pub pochtomat_number: Option<String>,
}

impl Address {
    pub fn warehouse(warehouse_number: String) -> Self {
        Address {
            warehouse_number: Some(warehouse_number),
            address_name: None,
            address_house: None,
            address_flat: None,
            pochtomat_number: None,
        }
    }
    pub fn address(address_name: String, address_house: String, address_flat: String) -> Self {
        Address {
            warehouse_number: None,
            address_name: Some(address_name),
            address_house: Some(address_house),
            address_flat: Some(address_flat),
            pochtomat_number: None,
        }
    }
    pub fn pochtomat(pochtomat_number: String) -> Self {
        Address {
            warehouse_number: None,
            address_name: None,
            address_house: None,
            address_flat: None,
            pochtomat_number: Some(pochtomat_number),
        }
    }
}

impl Recipient {
    pub fn new(
        city: String,
        first_name: String,
        last_name: String,
        phone: String,
        is_payer: bool,
        address: Address,
    ) -> Self {
        Recipient {
            city,
            first_name,
            last_name,
            phone,
            is_payer,
            address,
        }
    }

    // /// First argument this a counterparty, second contact counterparty
    // async fn get_refs(
    //     &self,
    //     api: &NovaposhtaRaw,
    // ) -> Result<(String, String), Box<dyn std::error::Error>> {
    //     let response = api
    //         .counterparty_create(
    //             self.first_name.clone(),
    //             self.last_name.clone(),
    //             self.phone.clone(),
    //             "Recipient".to_string(),
    //             None,
    //             self.middle_name.clone(),
    //         )
    //         .await?;
    //     println!("{:#?}", response);
    //     if response.success {
    //         let data = response.data().unwrap();

    //         let contact_person = data.ContactPerson.unwrap().data().unwrap();
    //         return Ok((data.Ref.unwrap(), contact_person.Ref.unwrap()));
    //     }
    //     Err(Box::new(NovaRequestError::new(
    //         "Novaposhta response not success".to_string(),
    //     )))
    // }
}

pub struct Cargo {
    cost: i32,
    weight: f32,
    options_seat: Option<NovaOptionsSeat>,
}

impl Cargo {
    pub fn new(cost: i32, weight: f32, options_seat: Option<NovaOptionsSeat>) -> Self {
        Cargo {
            cost,
            weight,
            options_seat,
        }
    }
}

#[derive(Debug)]
pub enum NovaServiceType {
    WarehouseWarehouse,
    WarehouseDoors,
    DoorsWarehouse,
    DoorsDoors,
}

#[derive(Debug)]
pub enum NovaPaymentMethod {
    Cash,
    NoCash,
}

impl fmt::Display for NovaPaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for NovaServiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::vec;
}
