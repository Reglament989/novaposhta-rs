use crate::properties::{NovaOptionsSeat, NovaRequest, NovaResponse, Properties};

use crate::properties::{NovaDocument, NovaRedeliveryCalculate};
use reqwest::Client;

pub struct NovaposhtaRaw {
    api_key: String,
    client: Client,
}

impl NovaposhtaRaw {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();
        NovaposhtaRaw { client, api_key }
    }
}

impl Default for NovaposhtaRaw {
    fn default() -> Self {
        let client = Client::new();
        NovaposhtaRaw {
            client,
            api_key: std::env::var("NOVAPOSHTA_KEY").unwrap(),
        }
    }
}

impl NovaposhtaRaw {
    pub async fn build_request(
        &self,
        model: &str,
        method: &str,
        properties: Properties,
    ) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let payload = NovaRequest::new(
            method.to_owned(),
            model.to_owned(),
            properties,
            self.api_key.to_owned(),
        );
        // BECAUSE THIS FUCKING DEVELOPERS FROM NP DOES NOT MIND OF DOCUMENTATION AND INSTEAD OF INT THEY GIVE STRING
        // use serde_json::Value;
        // use std::collections::HashMap;
        // let response1 = self
        //     .client
        //     .post("https://api.novaposhta.ua/v2.0/json/")
        //     .json(&payload)
        //     .send()
        //     .await?
        //     .json::<HashMap<String, Value>>()
        //     .await?;
        // println!("{:#?}", response1);
        let response: NovaResponse = self
            .client
            .post("https://api.novaposhta.ua/v2.0/json/")
            .json(&payload)
            .send()
            .await?
            .json::<NovaResponse>()
            .await?;
        Ok(response)
    }

    /// For more info ref: https://devcenter.novaposhta.ua/docs/services/556d7ccaa0fe4f08e8f7ce43/operations/556d885da0fe4f08e8f7ce46
    pub async fn get_cities(
        &self,
        city: Option<String>,
    ) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let mut props = Properties::default();
        // because getWarehouses works only with find.
        props.FindByString = city;
        let resp = self
            .build_request("AddressGeneral", "getCities", props)
            .await?;
        Ok(resp)
    }

    /// For more info ref:  https://devcenter.novaposhta.ua/docs/services/55702570a0fe4f0cf4fc53ed/operations/55702571a0fe4f0b64838913
    pub async fn get_types_of_payers(&self) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let props = Properties::default();
        // because getWarehouses works only with find.
        let resp = self
            .build_request("Common", "getTypesOfPayers", props)
            .await?;
        Ok(resp)
    }

    /// For more info ref: https://devcenter.novaposhta.ua/docs/services/55702570a0fe4f0cf4fc53ed/operations/55702571a0fe4f0b64838909
    pub async fn get_cargo_types(&self) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let props = Properties::default();
        // because getWarehouses works only with find.
        let resp = self.build_request("Common", "getCargoTypes", props).await?;
        Ok(resp)
    }

    pub async fn get_types_of_payers_for_redelivery(
        &self,
    ) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let props = Properties::default();
        // because getWarehouses works only with find.
        let resp = self
            .build_request("Common", "getTypesOfPayersForRedelivery", props)
            .await?;
        Ok(resp)
    }

    pub async fn get_service_types(&self) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let props = Properties::default();
        // because getWarehouses works only with find.
        let resp = self
            .build_request("Common", "getServiceTypes", props)
            .await?;
        Ok(resp)
    }

    pub async fn get_warehouses(
        &self,
        city_ref: Option<String>,
        city_name: Option<String>,
    ) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let mut props = Properties::default();
        props.CityRef = city_ref;
        props.CityName = city_name;
        let resp = self
            .build_request("AddressGeneral", "getWarehouses", props)
            .await?;
        Ok(resp)
    }

    pub async fn get_status_documents(
        &self,
        documents: Vec<NovaDocument>,
    ) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let mut props = Properties::default();
        props.Documents = Some(documents);
        let resp = self
            .build_request("TrackingDocument", "getStatusDocuments", props)
            .await?;
        Ok(resp)
    }

    pub async fn counterparty_create(
        &self,
        first_name: String,
        last_name: String,
        phone: String,
        counterparty_type: String,
        email: Option<String>,
        middle_name: Option<String>,
    ) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let mut props = Properties::default();
        props.FirstName = Some(first_name);
        props.LastName = Some(last_name);
        props.Phone = Some(phone);
        props.MiddleName = middle_name;
        props.Email = email;
        props.CounterpartyType = Some("PrivatePerson".to_owned());
        props.CounterpartyProperty = Some(counterparty_type.to_owned());
        let resp = self.build_request("Counterparty", "save", props).await?;
        Ok(resp)
    }

    pub async fn counterparty_search(
        &self,
        search_string: Option<String>,
        counterparty_type: String,
    ) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let mut props = Properties::default();
        props.FindByString = search_string;
        props.CounterpartyProperty = Some(counterparty_type.to_owned());
        let resp = self
            .build_request("Counterparty", "getCounterparties", props)
            .await?;
        Ok(resp)
    }

    pub async fn get_counterparty_contact_persons(
        &self,
        counterparty_ref: String,
    ) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let mut props = Properties::default();
        props.Ref = Some(counterparty_ref);
        let resp = self
            .build_request("Counterparty", "getCounterpartyContactPersons", props)
            .await?;
        Ok(resp)
    }

    /// No needed because counterparty_create returns this request into in field ContactPerson.
    pub async fn contact_person_create(
        &self,
        first_name: String,
        last_name: String,
        phone: String,
        counterparty_ref: String,
        middle_name: Option<String>,
    ) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let mut props = Properties::default();
        props.FirstName = Some(first_name);
        props.LastName = Some(last_name);
        props.Phone = Some(phone);
        props.MiddleName = middle_name.or(Some("".to_owned()));
        props.CounterpartyRef = Some(counterparty_ref);
        let resp = self.build_request("ContactPerson", "save", props).await?;
        Ok(resp)
    }

    pub async fn internet_document_create_warehouse(
        &self,
        payer_type: String,
        payment_method: String,
        cargo_type: String,
        weight: f32,
        service_type: String,
        seats_amount: i32,
        description: String,
        cost: i32,
        city_sender: String,
        sender: String,
        sender_address: String,
        contact_sender: String,
        senders_phone: String,
        recipient_city_name: String,
        warehouse_number: Option<String>,
        recipient_full_name: String,
        recipient_phone: String,
        date_time: String,
        option_seats: Option<Vec<NovaOptionsSeat>>,
        recipient_address: Option<String>,
        recipient_address_name: Option<String>,
        recipient_house_number: Option<String>,
        recipient_house_flat: Option<String>,
    ) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let mut props = Properties::default();
        props.OptionsSeat = option_seats;
        props.RecipientAddress = recipient_address;
        props.RecipientAddressName = recipient_address_name;
        props.RecipientHouse = recipient_house_number;
        props.RecipientFlat = recipient_house_flat;
        props.NewAddress = Some("1".to_string());
        props.PayerType = Some(payer_type);
        props.PaymentMethod = Some(payment_method);
        props.CargoType = Some(cargo_type);
        props.Weight = Some(weight.to_string());
        props.ServiceType = Some(service_type);
        props.SeatsAmount = Some(seats_amount.to_string());
        props.Description = Some(description);
        props.Cost = Some(cost.to_string());
        props.CitySender = Some(city_sender);
        props.Sender = Some(sender);
        props.SenderAddress = Some(sender_address);
        props.ContactSender = Some(contact_sender);
        props.SendersPhone = Some(senders_phone);
        props.RecipientCityName = Some(recipient_city_name);
        props.RecipientArea = Some("".to_string());
        props.RecipientAreaRegions = Some("".to_string());
        props.RecipientAddressName = warehouse_number;
        props.RecipientHouse = Some("".to_string());
        props.RecipientFlat = Some("".to_string());
        props.RecipientName = Some(recipient_full_name);
        props.RecipientType = Some("PrivatePerson".to_string());
        props.RecipientsPhone = Some(recipient_phone);
        props.DateTime = Some(date_time);

        let resp = self
            .build_request("InternetDocument", "save", props)
            .await?;
        Ok(resp)
    }

    pub async fn internet_document_delete(
        &self,
        document_ref: Vec<String>,
    ) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let mut props = Properties::default();
        props.DocumentRefs = Some(document_ref);
        let resp = self
            .build_request("InternetDocument", "delete", props)
            .await?;
        Ok(resp)
    }

    pub async fn get_document_price(
        &self,
        city_sender_ref: String,
        city_recipient_ref: String,
        weight: String,
        service_type: String,
        cost_of_parcel: String,
        cargo_type: String,
        seats_amount: String,
        date_time: Option<String>,
        redelivery_calculate: Option<NovaRedeliveryCalculate>,
        // pack_count: Option<i32>,
        // pack_ref: Option<i32>,
        // amount: Option<i32>,
        // cargo_details: Option<String>,
    ) -> Result<NovaResponse, Box<dyn std::error::Error>> {
        let mut props = Properties::default();
        props.CitySender = Some(city_sender_ref);
        props.CityRecipient = Some(city_recipient_ref);
        props.Weight = Some(weight);
        props.ServiceType = Some(service_type);
        props.Cost = Some(cost_of_parcel);
        props.CargoType = Some(cargo_type);
        props.SeatsAmount = Some(seats_amount);
        props.RedeliveryCalculate = redelivery_calculate;
        props.DateTime = date_time;

        let resp = self
            .build_request("InternetDocument", "getDocumentPrice", props)
            .await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    const KHARKIV_REF: &str = "db5c88e0-391c-11dd-90d9-001a92567626";
    use std::{env, vec};

    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn get_cities_test() {
        dotenv().ok();
        let nova = NovaposhtaRaw::default();
        let response = nova.get_cities(None).await.unwrap();
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn get_warehouses_test() {
        dotenv().ok();
        let nova = NovaposhtaRaw::default();
        let mut response = nova
            .get_warehouses(Some(KHARKIV_REF.to_owned()), None)
            .await
            .unwrap();
        if response.success {
            println!("{:?}", response.data.pop());
        };
    }

    #[tokio::test]
    async fn counterparty_create_test() {
        dotenv().ok();
        let nova = NovaposhtaRaw::default();
        let response = nova
            .counterparty_create(
                "Фелікс".to_owned(),
                "Яковлєв".to_owned(),
                "0997979789".to_owned(),
                "Recipient".to_string(),
                None,
                None,
            )
            .await
            .unwrap();
        if !response.success {
            println!("{:#?}", response);
        }
        let data = response.data().unwrap();
        let contact_person = data.ContactPerson.unwrap().data().unwrap();
        assert_ne!(data.Ref, contact_person.Ref);
        // println!("{:?}", response);
        println!("{:?}", response.data.first().unwrap().Description);
        println!("{:?}", response.data.first().unwrap().ContactPerson);
    }

    #[tokio::test]
    async fn contact_person_create_test() {
        dotenv().ok();
        let nova = NovaposhtaRaw::default();
        let response = nova
            .counterparty_create(
                "Фелікс".to_owned(),
                "Яковлєв".to_owned(),
                "0997979789".to_owned(),
                "Recipient".to_string(),
                None,
                None,
            )
            .await
            .unwrap();
        let data = response.data().unwrap();
        let contact_person = data.ContactPerson.unwrap().data().unwrap();
        assert_ne!(data.Ref, contact_person.Ref);
        // println!("{:?}", response);
        println!("{:?}", data.Ref);
        println!("{:?}", contact_person.Ref)
    }

    #[ignore = "Need alive ttn"]
    #[tokio::test]
    async fn get_status_documents_test() {
        dotenv().ok();
        let nova = NovaposhtaRaw::default();
        let response = nova
            .get_status_documents(vec![NovaDocument {
                DocumentNumber: env::var("TTN").unwrap(),
                Phone: env::var("PHONE").unwrap(),
            }])
            .await
            .unwrap();
        let data = response.data().unwrap();
        let address = data.RecipientAddress.unwrap();
        let date_shedule = data.ScheduledDeliveryDate.unwrap();
        let cargo_type = data.CargoType.unwrap();
        let status = data.Status.unwrap();
        println!("{:?}", vec![address, date_shedule, cargo_type, status]);
    }

    #[tokio::test]
    async fn get_document_price_test() {
        dotenv().ok();
        let nova = NovaposhtaRaw::default();
        let response = nova
            .get_document_price(
                KHARKIV_REF.to_string(),
                KHARKIV_REF.to_string(),
                "0.5".to_string(),
                "WarehouseWarehouse".to_string(),
                "250".to_string(),
                "Parcel".to_string(),
                1.to_string(),
                None,
                Some(NovaRedeliveryCalculate {
                    CargoType: "Money".to_string(),
                    Amount: "250".to_string(),
                }),
            )
            .await
            .unwrap();
        let data = response.data().unwrap();
        let cost_delivery = data.Cost.unwrap();
        let cost_redelivery = data.CostRedelivery.unwrap();
        println!("{:?}", cost_delivery + cost_redelivery);
    }

    #[tokio::test]
    async fn counterparty_search_test() {
        dotenv().ok();
        let nova = NovaposhtaRaw::default();
        let mut response = nova
            .counterparty_search(None, "Sender".to_string())
            .await
            .unwrap();
        if response.success {
            println!("{:?}", response.data.pop());
        };
    }

    #[tokio::test]
    async fn get_counterparty_contact_persons_test() {
        dotenv().ok();
        let nova = NovaposhtaRaw::default();
        let counter_ref = nova
            .counterparty_search(None, "Sender".to_string())
            .await
            .unwrap()
            .data()
            .unwrap()
            .Ref
            .unwrap();
        let mut response = nova
            .get_counterparty_contact_persons(counter_ref)
            .await
            .unwrap();
        println!("{:#?}", response);
        if response.success {
            println!("{:?}", response.data.pop());
        };
    }
}
