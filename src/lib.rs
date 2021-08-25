use std::fmt;

use properties::{
    BackwardDeliveryData, NovaDocument, NovaOptionsSeat, NovaRequestError, NovaResponseData,
};
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
    /// ```rust
    /// # async fn get_ttn() -> Result<(), Box<dyn std::error::Error>> {
    /// # use chrono::{Datelike, Duration};
    /// # use novaposhta::{NovaPaymentMethod, Novaposhta, CreateNewTtnPayload, Cargo, Address, Recipient, Sender};
    /// # use novaposhta::properties::BackwardDeliveryData;
    ///     let nova = Novaposhta::default();
    ///     let date_time = {
    ///         let now = chrono::Local::today() + Duration::days(1);
    ///         format!("{}.{}.{}", now.day(), now.month(), now.year())
    ///     };
    ///     let payload = CreateNewTtnPayload::new(
    ///         Recipient::new(
    ///             "Киев",
    ///             "имя",
    ///             Some("отчество"),
    ///             "фамилия",
    ///             "номер телефона",
    ///             true, // is_payer
    ///             Address::warehouse(14),
    ///         ),
    ///         Sender::new("Киев", "отделение", "номер телефона"),
    ///         date_time, // Time of
    ///         NovaPaymentMethod::Cash,
    ///         "Аксесуары".to_string(),
    ///         vec![Cargo::new(150, 0.5, None)],
    ///         "Parcel".to_string(),
    ///         Some(BackwardDeliveryData::money(150)),
    ///     );
    ///     nova.create_ttn(payload).await?;
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn create_ttn(
        &self,
        payload: CreateNewTtnPayload,
    ) -> Result<CreateNewTtnResponse, Box<dyn std::error::Error>> {
        let payer_type = if payload.recipient.is_payer {
            "Recipient".to_string()
        } else {
            "Sender".to_string()
        };
        let cargo_props = payload.get_props();
        let sender_city = self.get_city_ref(payload.sender.city.clone()).await?;
        let sender_props = self.get_first_sender().await?;
        let sender_address = self
            .get_warehouses_ref(
                payload.sender.warehouse_number.clone(),
                payload.sender.city.clone(),
            )
            .await?;
        let recipient_full_name = format!(
            "{} {} {}",
            payload.recipient.first_name,
            payload.recipient.last_name,
            payload.recipient.middle_name
        );
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
                    recipient_full_name,
                    payload.recipient.phone,
                    payload.date_time,
                    None,
                    None,
                    None,
                    None,
                    None,
                    payload.backward_delivery,
                )
                .await?;
            let data = response.data()?;
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
                    recipient_full_name,
                    payload.recipient.phone,
                    payload.date_time,
                    Some(options),
                    Some(pochtomat_ref),
                    None,
                    None,
                    None,
                    payload.backward_delivery,
                )
                .await?;
            println!("{:#?}", response);
            let data = response.data()?;
            return Ok(CreateNewTtnResponse {
                ttn: data.IntDocNumber.unwrap(),
                ttn_ref: data.Ref.unwrap(),
                cost_delivery: data.CostOnSite.unwrap(),
                estimated_delivery_date: data.EstimatedDeliveryDate.unwrap(),
            });
        } else if payload.recipient.address.address_name.is_some() {
            let service_type = NovaServiceType::WarehouseDoors;
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
                    recipient_full_name,
                    payload.recipient.phone,
                    payload.date_time,
                    None,
                    None,
                    payload.recipient.address.address_name,
                    payload.recipient.address.address_house,
                    payload.recipient.address.address_flat,
                    payload.backward_delivery,
                )
                .await?;
            let data = response.data()?;
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

    /// # Example
    /// ```rust
    /// # async fn get_ttn() -> Result<(), Box<dyn std::error::Error>> {
    /// # use novaposhta::{Novaposhta};
    ///     let nova = Novaposhta::default();
    ///     let payload = vec![
    ///         "TTN ref".to_string()
    ///     ];
    ///     nova.delete_ttn(payload).await?; // Always check if your document in delete list from response
    /// #   Ok(())
    /// # }
    /// ```
    /// For more info check <https://devcenter.novaposhta.ua/docs/services/556eef34a0fe4f02049c664e/operations/55701fa5a0fe4f0cf4fc53ec>
    pub async fn delete_ttn(
        &self,
        ttn_ref: Vec<String>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut deleted = vec![];
        let response = self.raw.internet_document_delete(ttn_ref).await?.data;
        for delete in response {
            deleted.push(delete.Ref.unwrap());
        }
        Ok(deleted)
    }

    /// # Example
    /// ```rust
    /// # async fn get_ttn() -> Result<(), Box<dyn std::error::Error>> {
    /// # use novaposhta::{Novaposhta};
    /// # use novaposhta::properties::NovaDocument;
    ///     let nova = Novaposhta::default();
    ///     let payload = vec![
    ///         NovaDocument::new("Number ttn".to_string(), "Phone of sender or recipient".to_string()),
    ///     ];
    ///     nova.track_ttn(payload).await?;
    /// #   Ok(())
    /// # }
    /// ```
    /// For more info check <https://devcenter.novaposhta.ua/docs/services/556eef34a0fe4f02049c664e/operations/55702cbba0fe4f0cf4fc53ee>
    pub async fn track_ttn(
        &self,
        ttns: Vec<NovaDocument>,
    ) -> Result<Vec<NovaResponseData>, Box<dyn std::error::Error>> {
        let data = self.raw.get_status_documents(ttns).await?.data;
        Ok(data)
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
    backward_delivery: Option<Vec<BackwardDeliveryData>>,
}

impl CreateNewTtnPayload {
    pub fn new(
        recipient: Recipient,
        sender: Sender,
        date_time: String,
        payment_method: NovaPaymentMethod,
        description: String,
        cargos: Vec<Cargo>,
        cargo_type: String,
        backward_delivery: Option<Vec<BackwardDeliveryData>>,
    ) -> Self {
        CreateNewTtnPayload {
            recipient,
            sender,
            date_time,
            payment_method,
            description,
            cargos,
            cargo_type,
            backward_delivery,
        }
    }
}

pub struct Sender {
    city: String,
    warehouse_number: String,
    phone: String,
}

impl Sender {
    pub fn new(city: &str, warehouse_number: &str, phone: &str) -> Self {
        Sender {
            city: city.to_owned(),
            warehouse_number: warehouse_number.to_owned(),
            phone: phone.to_owned(),
        }
    }
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
    middle_name: String,
    last_name: String,
    phone: String,
    is_payer: bool,
    address: Address,
}

impl Recipient {
    pub fn new(
        city: &str,
        first_name: &str,
        middle_name: Option<&str>,
        last_name: &str,
        phone: &str,
        is_payer: bool,
        address: Address,
    ) -> Self {
        Recipient {
            city: city.to_owned(),
            first_name: first_name.to_owned(),
            middle_name: middle_name.or(Some("")).unwrap().to_owned(),
            last_name: last_name.to_owned(),
            phone: phone.to_owned(),
            is_payer,
            address,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(default)]
pub struct Address {
    pub warehouse_number: Option<String>,
    pub address_name: Option<String>,
    pub address_house: Option<String>,
    pub address_flat: Option<String>,
    pub pochtomat_number: Option<String>,
}

impl Address {
    pub fn warehouse(warehouse_number: i32) -> Self {
        Address {
            warehouse_number: Some(warehouse_number.to_string()),
            address_name: None,
            address_house: None,
            address_flat: None,
            pochtomat_number: None,
        }
    }
    pub fn address(address_name: String, address_house: String, apartment_number: String) -> Self {
        Address {
            warehouse_number: None,
            address_name: Some(address_name),
            address_house: Some(address_house),
            address_flat: Some(apartment_number),
            pochtomat_number: None,
        }
    }
    pub fn pochtomat(pochtomat_number: i32) -> Self {
        Address {
            warehouse_number: None,
            address_name: None,
            address_house: None,
            address_flat: None,
            pochtomat_number: Some(pochtomat_number.to_string()),
        }
    }
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
    use chrono::{Datelike, Duration};
    use dotenv::dotenv;
    use std::env;

    #[tokio::test]
    async fn create_ttn_test() {
        dotenv().ok();
        let nova = Novaposhta::default();
        let date_time = {
            let now = chrono::Local::today() + Duration::days(1);
            format!("{}.{}.{}", now.day(), now.month(), now.year())
        };
        let payload = CreateNewTtnPayload::new(
            Recipient::new(
                env::var("TEST_CITY").unwrap().as_str(),
                env::var("TEST_FIRST_NAME").unwrap().as_str(),
                None,
                env::var("TEST_LAST_NAME").unwrap().as_str(),
                env::var("TEST_PHONE").unwrap().as_str(),
                true,
                Address::warehouse(14),
            ),
            Sender::new(
                env::var("TEST_CITY").unwrap().as_str(),
                env::var("TEST_WAREHOUSE").unwrap().as_str(),
                env::var("TEST_PHONE").unwrap().as_str(),
            ),
            date_time,
            NovaPaymentMethod::Cash,
            "Аксесуары".to_string(),
            vec![Cargo::new(150, 0.5, None)],
            "Parcel".to_string(),
            None,
        );
        let ttn_result = match nova.create_ttn(payload).await {
            Ok(value) => value,
            Err(error) => {
                println!("{:#?}", error);
                panic!("{}", error.to_string());
            }
        };

        nova.raw
            .internet_document_delete(vec![ttn_result.ttn_ref])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn create_ttn_pochtomat_test() {
        dotenv().ok();
        let nova = Novaposhta::default();
        let date_time = {
            let now = chrono::Local::today() + Duration::days(1);
            format!("{}.{}.{}", now.day(), now.month(), now.year())
        };
        let payload = CreateNewTtnPayload::new(
            Recipient::new(
                env::var("TEST_CITY").unwrap().as_str(),
                env::var("TEST_FIRST_NAME").unwrap().as_str(),
                None,
                env::var("TEST_LAST_NAME").unwrap().as_str(),
                env::var("TEST_PHONE").unwrap().as_str(),
                true,
                Address::pochtomat(6136),
            ),
            Sender::new(
                env::var("TEST_CITY").unwrap().as_str(),
                env::var("TEST_WAREHOUSE").unwrap().as_str(),
                env::var("TEST_PHONE").unwrap().as_str(),
            ),
            date_time,
            NovaPaymentMethod::Cash,
            "Аксесуары".to_string(),
            vec![Cargo::new(150, 0.5, Some(NovaOptionsSeat::default()))],
            "Parcel".to_string(),
            Some(BackwardDeliveryData::money(150)),
        );
        let ttn_result = match nova.create_ttn(payload).await {
            Ok(value) => value,
            Err(error) => {
                println!("{:#?}", error);
                panic!("{}", error.to_string());
            }
        };

        nova.raw
            .internet_document_delete(vec![ttn_result.ttn_ref])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn create_ttn_address_test() {
        dotenv().ok();
        let nova = Novaposhta::default();
        let date_time = {
            let now = chrono::Local::today() + Duration::days(1);
            format!("{}.{}.{}", now.day(), now.month(), now.year())
        };
        let payload = CreateNewTtnPayload::new(
            Recipient::new(
                env::var("TEST_CITY").unwrap().as_str(),
                env::var("TEST_FIRST_NAME").unwrap().as_str(),
                Some(env::var("TEST_MIDDLE_NAME").unwrap().as_str()),
                env::var("TEST_LAST_NAME").unwrap().as_str(),
                env::var("TEST_PHONE").unwrap().as_str(),
                true,
                Address::address(
                    env::var("TEST_ADDRESS_NAME").unwrap(),
                    env::var("TEST_ADDRESS_HOUSE").unwrap(),
                    env::var("TEST_ADDRESS_FLAT").unwrap(),
                ),
            ),
            Sender::new(
                env::var("TEST_CITY").unwrap().as_str(),
                env::var("TEST_WAREHOUSE").unwrap().as_str(),
                env::var("TEST_PHONE").unwrap().as_str(),
            ),
            date_time,
            NovaPaymentMethod::Cash,
            "Аксесуары".to_string(),
            vec![Cargo::new(150, 0.5, None)],
            "Parcel".to_string(),
            None,
        );
        let ttn_result = match nova.create_ttn(payload).await {
            Ok(value) => value,
            Err(error) => {
                println!("{:#?}", error);
                panic!("{}", error.to_string());
            }
        };

        nova.raw
            .internet_document_delete(vec![ttn_result.ttn_ref])
            .await
            .unwrap();
    }
}
