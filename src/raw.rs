use crate::{
    error::NovaRequestError,
    types::{
        NovaDeliveryDate, NovaDocumentPrice, NovaScanSheetCreate, NovaTTNFromList, NovaTime,
        ScanSheetListItem, ScanSheetRefsDelete,
    },
};
use chrono::Duration;
use log::debug;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::types::{
    Cargo, CargoSplit, NovaCity, NovaCounterParties, NovaDocument, NovaReferenceBooks, NovaRequest,
    NovaResponse, NovaServiceType, NovaTTN, NovaTTNCreate, NovaTTNDelete, NovaWarehouse, Recipient,
    Sender,
};

pub struct Novaposhta {
    api_key: String,
    client: Client,
}

impl Novaposhta {
    pub fn new(api_key: String) -> Self {
        let client = Client::new();
        Novaposhta { client, api_key }
    }
}

impl Default for Novaposhta {
    fn default() -> Self {
        let client = Client::new();
        Novaposhta {
            client,
            api_key: std::env::var("NOVAPOSHTA_KEY").unwrap(),
        }
    }
}

impl Novaposhta {
    pub async fn build_request<T>(
        &self,
        model: &str,
        method: &str,
        payload: serde_json::Value,
    ) -> Result<NovaResponse<T>, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned + Clone,
    {
        let payload: NovaRequest = NovaRequest::new(
            method.to_string(),
            model.to_string(),
            payload,
            self.api_key.clone(),
        );
        let response: NovaResponse<Value> = self
            .client
            .post("https://api.novaposhta.ua/v2.0/json/")
            .json(&payload)
            .send()
            .await?
            .json::<NovaResponse<Value>>()
            .await?;
        debug!("{:#?}", response);
        if response.success {
            // Exists better way but im dont know
            let return_value: NovaResponse<T> = NovaResponse {
                success: response.success,
                errors: response.errors,
                warnings: response.warnings,
                data: response
                    .data
                    .into_iter()
                    .map(|d| serde_json::from_value::<T>(d).unwrap())
                    .collect(),
            };
            Ok(return_value)
        } else {
            Err(Box::new(NovaRequestError::new(format!(
                "{:#?}",
                response.errors
            ))))
        }
    }

    pub async fn get_cities(
        &self,
        city: Option<String>,
    ) -> Result<NovaResponse<NovaCity>, Box<dyn std::error::Error>> {
        // because getWarehouses works only with find.
        let resp: NovaResponse<NovaCity> = self
            .build_request(
                "AddressGeneral",
                "getCities",
                json!({ "FindByString": city }),
            )
            .await?;
        Ok(resp)
    }

    pub async fn get_types_of_payers(
        &self,
    ) -> Result<NovaResponse<NovaReferenceBooks>, Box<dyn std::error::Error>> {
        let resp = self
            .build_request("Common", "getTypesOfPayers", json!({}))
            .await?;
        Ok(resp)
    }

    pub async fn get_cargo_types(
        &self,
    ) -> Result<NovaResponse<NovaReferenceBooks>, Box<dyn std::error::Error>> {
        let resp = self
            .build_request("Common", "getCargoTypes", json!({}))
            .await?;
        Ok(resp)
    }

    pub async fn get_types_of_payers_for_redelivery(
        &self,
    ) -> Result<NovaResponse<NovaReferenceBooks>, Box<dyn std::error::Error>> {
        let resp = self
            .build_request("Common", "getTypesOfPayersForRedelivery", json!({}))
            .await?;
        Ok(resp)
    }

    pub async fn get_service_types(
        &self,
    ) -> Result<NovaResponse<NovaReferenceBooks>, Box<dyn std::error::Error>> {
        let resp = self
            .build_request("Common", "getServiceTypes", json!({}))
            .await?;
        Ok(resp)
    }

    pub async fn get_warehouses(
        &self,
        city: Option<String>,
        warehouse: Option<String>,
    ) -> Result<NovaResponse<NovaWarehouse>, Box<dyn std::error::Error>> {
        let payload: Value;
        match city {
            Some(find) => {
                let is_uuid = uuid::Uuid::parse_str(&find).is_ok();
                if is_uuid {
                    payload = json!({ "CityRef": find, "FindByString": warehouse });
                } else {
                    payload = json!({ "CityName": find, "FindByString": warehouse });
                }
            }
            None => payload = json!({ "FindByString": warehouse }),
        }
        let resp = self
            .build_request("AddressGeneral", "getWarehouses", payload)
            .await?;
        Ok(resp)
    }

    pub async fn get_status_documents(
        &self,
        documents: Vec<NovaDocument>,
    ) -> Result<NovaResponse<NovaTTN>, Box<dyn std::error::Error>> {
        let resp = self
            .build_request(
                "TrackingDocument",
                "getStatusDocuments",
                json!({ "Documents": documents }),
            )
            .await?;
        Ok(resp)
    }

    pub async fn counterparty_search(
        &self,
        search_string: Option<String>,
        counterparty_type: &str,
    ) -> Result<NovaResponse<NovaCounterParties>, Box<dyn std::error::Error>> {
        let resp = self
            .build_request(
                "Counterparty",
                "getCounterparties",
                json!({
                    "CounterpartyProperty": counterparty_type,
                    "FindByString": search_string
                }),
            )
            .await?;
        Ok(resp)
    }

    pub async fn get_counterparty_contact_persons(
        &self,
        counterparty_ref: String,
    ) -> Result<NovaResponse<NovaCounterParties>, Box<dyn std::error::Error>> {
        let resp = self
            .build_request(
                "Counterparty",
                "getCounterpartyContactPersons",
                json!({ "Ref": counterparty_ref }),
            )
            .await?;
        Ok(resp)
    }

    pub async fn internet_document_create(
        &self,
        recipient: Recipient,
        sender: Sender,
        cargos: Vec<Cargo>,
        date_of_send: chrono::DateTime<chrono::Local>,
    ) -> Result<NovaResponse<NovaTTNCreate>, Box<dyn std::error::Error>> {
        let seats_amount = cargos.len();
        let (weight, price, to_payment, description) = cargos.into_ttn_values();
        let payer = if recipient.is_payer {
            "Recipient"
        } else {
            "Sender"
        };
        let (sender_ref, sender_contact_ref) = {
            let sender = self.counterparty_search(None, "Sender").await?;
            let sender_contact = self
                .get_counterparty_contact_persons(sender.data().clone().unwrap().Ref)
                .await?;
            (
                sender.data().unwrap().Ref,
                sender_contact.data().unwrap().Ref,
            )
        };
        let backward_delivery = if to_payment > 0 {
            vec![json!({
                "PayerType": "Recipient",
                "CargoType": "Money",
                "RedeliveryString": to_payment
            })]
        } else {
            vec![json!({})]
        };
        let payload = json!({
            "NewAddress": "1",
            "PayerType": payer,
            "Weight": weight,
            "CargoType": "Parcel",
            "ServiceType": recipient.address.service_type,
            "SeatsAmount": seats_amount,
            "Description": description,
            "Cost": price,
            "CitySender": sender.city_ref,
            "SenderAddress": sender.warehouse_ref,
            "Sender": sender_ref,
            "ContactSender": sender_contact_ref,
            "SendersPhone": sender.phone,
            "RecipientAddressName": recipient.address.address_name.unwrap_or(recipient.address.warehouse_number.unwrap()),
            "RecipientHouse": recipient.address.address_house,
            "RecipientFlat": recipient.address.address_flat,
            "RecipientCityName": recipient.city_name,
            "RecipientName": recipient.full_name,
            "RecipientType": "PrivatePerson",
            "RecipientsPhone": recipient.phone,
            "DateTime": date_of_send.into_ttn_time(),
            "PaymentMethod": "Cash",
            "BackwardDeliveryData": backward_delivery
        });
        let resp = self
            .build_request("InternetDocument", "save", payload)
            .await?;
        Ok(resp)
    }

    pub async fn internet_document_delete(
        &self,
        document_refs: Vec<String>,
    ) -> Result<NovaResponse<NovaTTNDelete>, Box<dyn std::error::Error>> {
        let resp = self
            .build_request(
                "InternetDocument",
                "delete",
                json!({ "DocumentRefs": document_refs }),
            )
            .await?;
        Ok(resp)
    }

    pub async fn get_document_price(
        &self,
        sender_city_ref: String,
        recipient_city_ref: String,
        service_type: NovaServiceType,
        cargos: Vec<Cargo>,
    ) -> Result<NovaResponse<NovaDocumentPrice>, Box<dyn std::error::Error>> {
        let seats_amount = cargos.len();
        let (weight, price, to_payment, _) = cargos.into_ttn_values();

        let redelivery_calculate = if to_payment > 0 {
            json!({
                "CargoType": "Money",
                "Amount": to_payment
            })
        } else {
            json!({})
        };

        let now = chrono::Local::now() + Duration::weeks(1);

        let resp = self
            .build_request(
                "InternetDocument",
                "getDocumentPrice",
                json!({
                    "CitySender": sender_city_ref,
                    "CityRecipient": recipient_city_ref,
                    "Weight": weight,
                    "ServiceType": service_type,
                    "Cost": price,
                    "CargoType": "Parcel",
                    "SeatsAmount": seats_amount,
                    "RedeliveryCalculate": redelivery_calculate,
                    "DateTime": now.into_ttn_time()
                }),
            )
            .await?;
        Ok(resp)
    }

    pub async fn get_document_list(
        &self,
        date_time_to: Option<String>,
        date_time_from: Option<String>,
        date_time_of_ttn: Option<String>,
    ) -> Result<NovaResponse<NovaTTNFromList>, Box<dyn std::error::Error>> {
        let resp = self
            .build_request(
                "InternetDocument",
                "getDocumentList",
                json!({
                    "DateTimeTo": date_time_to,
                    "DateTimeFrom": date_time_from,
                    "DateTime": date_time_of_ttn
                }),
            )
            .await?;
        Ok(resp)
    }

    pub async fn get_document_delivery_date(
        &self,
        send_date: String,
        service_type: NovaServiceType,
        city_sender_ref: String,
        city_recipient_ref: String,
    ) -> Result<NovaResponse<NovaDeliveryDate>, Box<dyn std::error::Error>> {
        let resp = self
            .build_request(
                "InternetDocument",
                "getDocumentDeliveryDate",
                json!({
                    "DateTime": send_date,
                    "ServiceType": service_type,
                    "CitySender": city_sender_ref,
                    "CityRecipient": city_recipient_ref
                }),
            )
            .await?;

        Ok(resp)
    }

    pub async fn scan_sheet_insert_documents(
        &self,
        document_refs: Vec<String>,
        add_to_exists_registy: Option<String>,
    ) -> Result<NovaResponse<NovaScanSheetCreate>, Box<dyn std::error::Error>> {
        let resp = self
            .build_request(
                "ScanSheet",
                "insertDocuments",
                json!({
                    "DocumentRefs": document_refs,
                    "Ref": add_to_exists_registy
                }),
            )
            .await?;

        Ok(resp)
    }

    pub async fn scan_sheet_delete(
        &self,
        scan_sheet_refs: Vec<String>,
    ) -> Result<NovaResponse<ScanSheetRefsDelete>, Box<dyn std::error::Error>> {
        let resp = self
            .build_request(
                "ScanSheet",
                "deleteScanSheet",
                json!({
                    "ScanSheetRefs": scan_sheet_refs,
                }),
            )
            .await?;

        Ok(resp)
    }

    pub async fn scan_sheet_delete_ttns(
        &self,
        documents_refs: Vec<String>,
    ) -> Result<NovaResponse<ScanSheetRefsDelete>, Box<dyn std::error::Error>> {
        let resp = self
            .build_request(
                "ScanSheet",
                "removeDocuments",
                json!({
                    "DocumentRefs": documents_refs,
                }),
            )
            .await?;

        Ok(resp)
    }

    pub async fn get_scan_sheet_list(
        &self,
    ) -> Result<NovaResponse<ScanSheetListItem>, Box<dyn std::error::Error>> {
        let resp = self
            .build_request("ScanSheet", "getScanSheetList", json!({}))
            .await?;

        Ok(resp)
    }

    pub fn internet_document_print(&self, ttns: Vec<String>) -> String {
        format!(
            "https://my.novaposhta.ua/orders/printDocument/orders[]/{}/type/pdf/apiKey/{}",
            ttns.join(","),
            self.api_key
        )
    }

    // pub fn scan_sheet_print(&self, scan_sheet_ref: String) -> String {
    //     format!(
    //         "https://my.novaposhta.ua/scanSheet/printScanSheet/refs[]/{}/type/pdf/apiKey/{}",
    //         scan_sheet_ref, self.api_key
    //     )
    // }

    pub async fn scan_sheet_print(
        &self,
        scan_sheet_ref: String,
    ) -> Result<bytes::Bytes, Box<dyn std::error::Error>> {
        let payload: NovaRequest = NovaRequest::new(
            "printFull".to_string(),
            "InternetDocument".to_string(),
            json!({
                "printForm": "ScanSheet",
                "ScanSheetRefs": vec![scan_sheet_ref],
                "Type": "pdf",
                "PrintOrientation": "portrait"
            }),
            self.api_key.clone(),
        );

        let resp = self
            .client
            .post("https://api.novaposhta.ua/v2.0/json/")
            .json(&payload)
            .send()
            .await?;
        // let mut file = std::fs::File::create("./test.pdf")?;
        // let mut content = Cursor::new();
        // std::io::copy(&mut content, &mut file)?;

        Ok(resp.bytes().await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Address, NovaOptionsSeat, NovaParcelWightStandarts};

    use super::*;
    use chrono::Duration;
    use dotenv::dotenv;

    const KHARKIV_REF: &str = "db5c88e0-391c-11dd-90d9-001a92567626";

    #[tokio::test]
    async fn get_warehouses_test() {
        dotenv().ok();
        let nova = Novaposhta::default();
        let warehouses = nova
            .get_warehouses(Some(KHARKIV_REF.to_string()), Some("14".to_string()))
            .await
            .unwrap();
        println!("{:#?}", warehouses);
    }

    #[tokio::test]
    async fn get_statuses_test() {
        dotenv().ok();

        let nova = Novaposhta::default();
        let ttns = nova
            .get_status_documents(vec![NovaDocument::new("2045045800000".to_string(), None)])
            .await
            .unwrap();
        println!("{:#?}", ttns);
    }

    #[tokio::test]
    async fn get_delivery_date_test() {
        dotenv().ok();

        let nova = Novaposhta::default();
        let now = chrono::Local::now() + Duration::days(3);
        let ttns = nova
            .get_document_delivery_date(
                now.into_ttn_time(),
                NovaServiceType::WarehouseDoors,
                KHARKIV_REF.to_string(),
                KHARKIV_REF.to_string(),
            )
            .await
            .unwrap();
        println!("{:#?}", ttns);
    }

    #[tokio::test]
    async fn get_document_list_test() {
        dotenv().ok();

        let nova = Novaposhta::default();
        let ttns = nova.get_document_list(None, None, None).await.unwrap();
        println!("{:#?}", ttns);
    }

    #[tokio::test]
    async fn scan_sheet_insert_test() {
        dotenv().ok();

        let nova = Novaposhta::default();
        let scan_sheet = nova
            .scan_sheet_insert_documents(
                vec!["436c1ba6-3373-11ec-8513-b88303659df5".to_string()],
                None,
            )
            .await
            .unwrap();
        println!("{:#?}", scan_sheet);
    }

    #[tokio::test]
    async fn scan_sheet_print_test() {
        dotenv().ok();

        let nova = Novaposhta::default();
        let _ = nova
            .scan_sheet_print("f23f5752-3377-11ec-8513-b88303659df5".to_string())
            .await
            .unwrap();
        println!("OK!");
    }

    #[tokio::test]
    async fn get_price_document_test() {
        dotenv().ok();
        let nova = Novaposhta::default();
        let ttns = nova
            .get_document_price(
                KHARKIV_REF.to_string(),
                KHARKIV_REF.to_string(),
                NovaServiceType::WarehouseWarehouse,
                vec![Cargo::new(
                    150,
                    NovaOptionsSeat::from(NovaParcelWightStandarts::UpToHalfKilogram),
                    true,
                    "Аксесуары".to_string(),
                )],
            )
            .await
            .unwrap();
        println!("{:#?}", ttns);
    }

    #[tokio::test]
    async fn internet_document_create_test() {
        dotenv().ok();
        let nova = Novaposhta::default();
        let sender_warehouse_ref = nova
            .get_warehouses(Some(KHARKIV_REF.to_string()), Some("14".to_string()))
            .await
            .unwrap()
            .data()
            .unwrap()
            .Ref;
        let sender = Sender::new(KHARKIV_REF, &sender_warehouse_ref, "+380991234567");
        let recipient_address = Address::warehouse(14);
        let recipient = Recipient::new(
            "Киев".to_string(),
            "Тест Тест Тест".to_string(),
            "+380991234567",
            true,
            recipient_address,
        );
        let now = chrono::Local::now() + Duration::days(3);
        let ttn = nova
            .internet_document_create(
                recipient,
                sender,
                vec![Cargo::new(
                    150,
                    NovaOptionsSeat::new(0.5, 20, 20, 5, 0.5),
                    true,
                    "Аксесуары".to_string(),
                )],
                now,
            )
            .await
            .unwrap();
        println!("{:#?}", ttn);
        let delete_response = nova
            .internet_document_delete(vec![ttn.data().unwrap().Ref])
            .await
            .unwrap();
        println!("{:#?}", delete_response);
    }
}

// impl From<Vec<Value>> for NovaResponsePublic<NovaCities> {
//     fn from(data: Vec<Value>) -> Self {
//         NovaResponsePublic {
//             data: NovaCities {
//                 Description: data["Description"],
//                 DescriptionRU: data["DescriptionRU"],
//                 Ref: data["Ref"],
//             },
//         }
//     }
// }

// impl Vec<T> for From<Vec<Value>> {}
