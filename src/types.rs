// use core::fmt;

// use chrono::{DateTime, Datelike, TimeZone};
// use serde::{Deserialize, Serialize};
// use serde_json::Value;

// #[derive(Serialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct NovaRequest {
//     pub model_name: String,
//     pub called_method: String,
//     pub method_properties: serde_json::Value,
//     pub api_key: String,
// }

// impl NovaRequest {
//     pub fn new(
//         called_method: String,
//         model_name: String,
//         method_properties: serde_json::Value,
//         api_key: String,
//     ) -> Self {
//         NovaRequest {
//             called_method,
//             model_name,
//             method_properties,
//             api_key,
//         }
//     }
// }

// #[derive(Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct NovaCity {
//     pub description: String,
//     pub r#ref: String,
// }

// #[derive(Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct NovaReferenceBooks {
//     pub description: String,
//     pub r#ref: String,
// }

// #[derive(Serialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct NovaDocument {
//     pub document_number: String,
//     pub phone: String,
// }

// impl NovaDocument {
//     pub fn new(ttn: String, phone: Option<String>) -> Self {
//         NovaDocument {
//             document_number: ttn,
//             phone: phone.unwrap_or_default(),
//         }
//     }
// }
// #[derive(Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct NovaTTNCreate {
//     pub int_doc_number: String,
//     pub r#ref: String,
//     pub cost_on_site: i32,
//     pub estimated_delivery_date: String,
// }

// #[derive(Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct NovaTTNDelete {
//     pub r#ref: String,
// }

// #[derive(Deserialize, Debug, Clone, Default)]
// #[serde(default)]
// pub struct NovaTTN {
//     pub Number: String,
//     pub Redelivery: String,
//     pub RedeliverySum: String,
//     pub RedeliveryNum: String,
//     pub RedeliveryPayer: String,
//     pub DateCreated: String,
//     pub DocumentWeight: f32,
//     pub CheckWeight: String,
//     pub DocumentCost: i32,
//     pub PayerType: String,
//     pub RecipientFullNameEW: String,
//     pub RecipientDateTime: String,
//     pub ScheduledDeliveryDate: String,
//     pub PaymentMethod: String,
//     pub CargoDescriptionString: String,
//     pub CitySender: String,
//     pub CityRecipient: String,
//     pub WarehouseRecipient: String,
//     pub ServiceType: String,
//     pub WarehouseRecipientNumber: String,
//     pub UndeliveryReasons: String,
//     pub PhoneRecipient: String,
//     pub RecipientAddress: String,
//     pub PaymentStatus: String,
//     pub PaymentStatusDate: String,
//     pub AmountToPay: String,
//     pub AmountPaid: String,
//     pub Status: String,
//     pub StatusCode: String,
// }

// #[allow(non_snake_case)]
// #[derive(Deserialize, Debug, Clone, Default)]
// #[serde(default)]
// pub struct NovaTTNFromList {
//     pub CityRecipient: String,
//     pub CityRecipientDescription: String,
//     pub CitySender: String,
//     pub CitySenderDescription: String,
//     pub ContactRecipient: String,
//     pub ContactSender: String,
//     pub Cost: String,
//     pub CostOnSite: String,
//     pub CreateTime: String,
//     pub EstimatedDeliveryDate: String,
//     pub IntDocNumber: String,
//     pub State: String,
//     pub StateId: String,
//     pub StateName: String,
//     pub RecipientContactPhone: String,
// }

// #[allow(non_snake_case)]
// #[derive(Deserialize, Debug, Clone)]
// pub struct NovaCounterParties {
//     pub Description: String,
//     pub Ref: String,
//     pub FirstName: String,
//     pub LastName: String,
//     pub MiddleName: String,
// }

// #[allow(non_snake_case)]
// #[derive(Deserialize, Debug, Clone)]
// pub struct NovaDocumentPrice {
//     pub Cost: i32,
//     pub CostRedelivery: i32,
//     pub AssessedCost: i32,
// }

// #[allow(non_snake_case)]
// #[derive(Deserialize, Debug, Clone)]
// pub struct NovaScanSheetCreate {
//     pub Ref: String,
//     pub Number: String,
//     pub Date: String,
// }

// #[allow(non_snake_case)]
// #[derive(Deserialize, Debug, Clone)]
// pub struct ScanSheetRefsDelete {
//     pub ScanSheetRefsDelete: ScanSheetRefsDeleteData,
// }

// #[allow(non_snake_case)]
// #[derive(Deserialize, Debug, Clone)]
// pub struct ScanSheetRefsDeleteData {
//     pub Ref: String,
//     pub Error: String,
//     pub Number: String,
// }

// #[allow(non_snake_case)]
// #[derive(Deserialize, Debug, Clone)]
// pub struct ScanSheetListItem {
//     pub Ref: String,
//     pub Number: String,
//     pub DateTime: String,
//     pub Printed: String,
// }

// #[allow(non_snake_case)]
// #[derive(Deserialize, Debug, Clone)]
// pub struct NovaDeliveryDate {
//     pub DeliveryDate: NovaDeliveryDateData,
// }

// #[allow(non_snake_case)]
// #[derive(Deserialize, Debug, Clone)]
// pub struct NovaDeliveryDateData {
//     pub date: String,
//     pub timezone: String,
// }

// pub struct Sender {
//     pub city_ref: String,
//     pub warehouse_ref: String,
//     pub phone: String,
// }

// impl Sender {
//     pub fn new(city_ref: &str, warehouse_ref: &str, phone: &str) -> Self {
//         Sender {
//             city_ref: city_ref.to_owned(),
//             warehouse_ref: warehouse_ref.to_owned(),
//             phone: phone.to_owned(),
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Address {
//     pub warehouse_number: Option<String>,
//     pub address_name: Option<String>,
//     pub address_house: Option<String>,
//     pub address_flat: Option<String>,
//     pub pochtomat_number: Option<String>,
//     pub service_type: NovaServiceType,
// }

// impl Address {
//     pub fn warehouse(warehouse_number: i32) -> Self {
//         Address {
//             warehouse_number: Some(warehouse_number.to_string()),
//             address_name: None,
//             address_house: None,
//             address_flat: None,
//             pochtomat_number: None,
//             service_type: NovaServiceType::WarehouseWarehouse,
//         }
//     }
//     pub fn address(address_name: String, address_house: String, apartment_number: String) -> Self {
//         Address {
//             warehouse_number: None,
//             address_name: Some(address_name),
//             address_house: Some(address_house),
//             address_flat: Some(apartment_number),
//             pochtomat_number: None,
//             service_type: NovaServiceType::WarehouseDoors,
//         }
//     }
//     pub fn pochtomat(pochtomat_number: i32) -> Self {
//         Address {
//             warehouse_number: None,
//             address_name: None,
//             address_house: None,
//             address_flat: None,
//             pochtomat_number: Some(pochtomat_number.to_string()),
//             service_type: NovaServiceType::WarehouseWarehouse,
//         }
//     }
// }

// pub struct Recipient {
//     pub city_name: String,
//     pub full_name: String,
//     pub phone: String,
//     pub is_payer: bool,
//     pub address: Address,
// }

// impl Recipient {
//     pub fn new(
//         city: String,
//         full_name: String,
//         phone: &str,
//         is_payer: bool,
//         address: Address,
//     ) -> Self {
//         Recipient {
//             city_name: city.to_owned(),
//             full_name,
//             phone: phone.to_owned(),
//             is_payer,
//             address,
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub struct Cargo {
//     pub cost: i32,
//     pub options_seat: NovaOptionsSeat,
//     pub payment_on_delivery: bool,
//     pub description: String,
// }

// impl Cargo {
//     pub fn new(cost: i32, options: NovaOptionsSeat, payment: bool, description: String) -> Self {
//         Cargo {
//             cost,
//             options_seat: options,
//             payment_on_delivery: payment,
//             description,
//         }
//     }
// }
// pub trait CargoSplit {
//     fn into_ttn_values(&self) -> (f32, i32, i32, String);
// }

// impl CargoSplit for Vec<Cargo> {
//     fn into_ttn_values(&self) -> (f32, i32, i32, String) {
//         let mut total = 0f32;
//         let mut price = 0;
//         let mut to_payment_ammount = 0;
//         let mut description: String = "".to_string();
//         for c in self.into_iter() {
//             total += c.options_seat.weight;
//             price += c.cost;
//             description = c.description.clone();
//             if c.payment_on_delivery {
//                 to_payment_ammount += c.cost;
//             }
//         }
//         (total, price, to_payment_ammount, description)
//     }
// }

// pub trait NovaTime {
//     fn into_ttn_time(&self) -> String;
// }

// impl<Tz: TimeZone> NovaTime for DateTime<Tz> {
//     fn into_ttn_time(&self) -> String {
//         format!("{}.{}.{}", self.day(), self.month(), self.year())
//     }
// }

// #[allow(non_snake_case)]
// #[derive(Serialize, Debug, Clone)]
// #[serde(default)]
// pub struct NovaOptionsSeat {
//     pub Volume: f32,
//     pub Width: i32,
//     pub Length: i32,
//     pub Height: i32,
//     pub weight: f32,
// }

// impl NovaOptionsSeat {
//     pub fn new(volume: f32, width: i32, lenght: i32, height: i32, weight: f32) -> Self {
//         NovaOptionsSeat {
//             Volume: volume,
//             Width: width,
//             Length: lenght,
//             Height: height,
//             weight,
//         }
//     }
// }

// pub enum NovaParcelWightStandarts {
//     UpToHalfKilogram,
// }

// impl From<NovaParcelWightStandarts> for NovaOptionsSeat {
//     fn from(s: NovaParcelWightStandarts) -> Self {
//         match s {
//             NovaParcelWightStandarts::UpToHalfKilogram => NovaOptionsSeat {
//                 Volume: 0.5,
//                 Width: 20,
//                 Length: 20,
//                 Height: 5,
//                 weight: 0.5,
//             },
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum NovaServiceType {
//     WarehouseWarehouse,
//     WarehouseDoors,
//     DoorsWarehouse,
//     DoorsDoors,
// }

// #[derive(Debug)]
// pub enum NovaPaymentMethod {
//     Cash,
//     NoCash,
// }

// impl fmt::Display for NovaPaymentMethod {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//         // or, alternatively:
//         // fmt::Debug::fmt(self, f)
//     }
// }

// impl fmt::Display for NovaServiceType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//         // or, alternatively:
//         // fmt::Debug::fmt(self, f)
//     }
// }
