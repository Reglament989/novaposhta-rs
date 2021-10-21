use std::{error::Error, fmt, vec};

use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Default)]
#[serde(default)]
pub struct Properties {
    pub Ref: Option<String>,
    pub FindByString: Option<String>,
    pub CityName: Option<String>,
    pub CityRef: Option<String>,
    pub Page: Option<String>,
    pub Limit: Option<String>,
    pub FirstName: Option<String>,
    pub MiddleName: Option<String>,
    pub LastName: Option<String>,
    pub Phone: Option<String>,
    pub Email: Option<String>,
    pub CounterpartyType: Option<String>,
    pub CounterpartyProperty: Option<String>,
    pub CounterpartyRef: Option<String>,
    pub NewAddress: Option<String>,
    pub PayerType: Option<String>,
    pub PaymentMethod: Option<String>,
    pub CargoType: Option<String>,
    // Total volume, cbm (min - 0.0004), mandatory, if OptionsSeat values are not specified
    pub VolumeGeneral: Option<String>,
    pub OptionsSeat: Option<Vec<NovaOptionsSeat>>,
    pub Weight: Option<String>,
    pub ServiceType: Option<String>,
    pub SeatsAmount: Option<String>,
    pub Description: Option<String>,
    pub Cost: Option<String>,
    // Get it from Novaposhta.get_cities
    pub CitySender: Option<String>,
    // Get it from Novaposhta.counterparty_create.data()?.Ref
    pub Sender: Option<String>,
    // Get it from Novaposhta.get_warehouses
    pub SenderAddress: Option<String>,
    // Get it from Novaposhta.counterparty_create.data().ContactPerson.data().Ref
    pub ContactSender: Option<String>,
    pub SendersPhone: Option<String>,
    // Get it from Novaposhta.get_cities
    pub CityRecipient: Option<String>,
    // Get it from Novaposhta.counterparty_create.data()?.Ref
    pub Recipient: Option<String>,
    // Get it from Novaposhta.get_warehouses
    pub RecipientAddress: Option<String>,
    // Get it from Novaposhta.counterparty_create.data().ContactPerson.data().Ref
    pub ContactRecipient: Option<String>,
    pub RecipientsPhone: Option<String>,
    // need be d.M.Y
    pub DateTime: Option<String>,
    pub DateTimeFrom: Option<String>,
    pub DateTimeTo: Option<String>,
    pub Documents: Option<Vec<NovaDocument>>,
    pub DocumentRefs: Option<Vec<String>>,
    pub RedeliveryCalculate: Option<NovaRedeliveryCalculate>,
    pub RecipientAddressName: Option<String>,
    pub RecipientCityName: Option<String>,
    pub RecipientArea: Option<String>,
    pub RecipientAreaRegions: Option<String>,
    pub RecipientHouse: Option<String>,
    pub RecipientFlat: Option<String>,
    pub RecipientName: Option<String>,
    pub RecipientType: Option<String>,
    pub Counterparty: Option<String>,
    pub BackwardDeliveryData: Option<Vec<BackwardDeliveryData>>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct BackwardDeliveryData {
    pub PayerType: NovaPayerType,
    pub CargoType: NovaBackwardCargo,
    pub RedeliveryString: String,
}

impl BackwardDeliveryData {
    pub fn money(amount: i32) -> Vec<BackwardDeliveryData> {
        vec![BackwardDeliveryData {
            PayerType: NovaPayerType::Recipient,
            CargoType: NovaBackwardCargo::Money,
            RedeliveryString: amount.to_string(),
        }]
    }
}

#[derive(Serialize, Debug)]
pub enum NovaPayerType {
    Sender,
    Recipient,
}

impl fmt::Display for NovaPayerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Debug)]
pub enum NovaBackwardCargo {
    Money,
    // CreditDocuments,
    // SignedDocuments,
    // Documents
}

impl fmt::Display for NovaBackwardCargo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
#[serde(default)]
pub struct NovaDocument {
    pub DocumentNumber: String,
    pub Phone: String,
}

impl NovaDocument {
    pub fn new(ttn: String, phone: String) -> Self {
        NovaDocument {
            DocumentNumber: ttn,
            Phone: phone,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone)]
#[serde(default)]
pub struct NovaOptionsSeat {
    volumetricVolume: String,
    volumetricWidth: String,
    volumetricLength: String,
    volumetricHeight: String,
    weight: String,
}

#[allow(non_snake_case)]
impl NovaOptionsSeat {
    pub fn new(
        volumetricVolume: f32,
        volumetricWidth: f32,
        volumetricLenght: f32,
        volumetricHeight: f32,
        weight: f32,
    ) -> Self {
        NovaOptionsSeat {
            volumetricVolume: volumetricVolume.to_string(),
            volumetricWidth: volumetricWidth.to_string(),
            volumetricLength: volumetricLenght.to_string(),
            volumetricHeight: volumetricHeight.to_string(),
            weight: weight.to_string(),
        }
    }
}

impl Default for NovaOptionsSeat {
    fn default() -> Self {
        NovaOptionsSeat {
            volumetricVolume: 0.5.to_string(),
            volumetricWidth: 20.to_string(),
            volumetricLength: 20.to_string(),
            volumetricHeight: 5.to_string(),
            weight: 0.5.to_string(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct NovaRequest {
    pub modelName: String,
    pub calledMethod: String,
    pub methodProperties: Properties,
    pub apiKey: String,
}

#[allow(non_snake_case)]
impl NovaRequest {
    pub fn new(
        calledMethod: String,
        modelName: String,
        methodProperties: Properties,
        apiKey: String,
    ) -> Self {
        NovaRequest {
            calledMethod,
            modelName,
            methodProperties,
            apiKey,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NovaResponse {
    pub success: bool,
    pub data: Vec<NovaResponseData>,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl NovaResponse {
    pub fn data(&self) -> Result<NovaResponseData, NovaRequestError> {
        let d = self.data.clone();
        let returnable = d.first();
        if returnable.is_none() {
            return Err(NovaRequestError::new(format!(
                "{} {:#?}",
                "NovaResponseData with lenght 0".to_owned(),
                self.errors
            )));
        }
        Ok(returnable.unwrap().clone())
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default ,Clone)]
pub struct DeliveryDate {
    pub date: String,
}


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(default)]
pub struct NovaResponseData {
    pub DeliveryDate: Option<DeliveryDate>,
    pub FirstName: Option<String>,
    pub MiddleName: Option<String>,
    pub LastName: Option<String>,
    pub SiteKey: Option<String>,
    pub Ref: Option<String>,
    pub CityRef: Option<String>,
    pub Description: Option<String>,
    pub DescriptionRu: Option<String>,
    pub CityDescription: Option<String>,
    pub CityDescriptionRu: Option<String>,
    pub Delivery1: Option<String>,
    pub Delivery2: Option<String>,
    pub Delivery3: Option<String>,
    pub Delivery4: Option<String>,
    pub Delivery5: Option<String>,
    pub Delivery6: Option<String>,
    pub Delivery7: Option<String>,
    pub Area: Option<String>,
    pub SettlementType: Option<String>,
    pub IsBranch: Option<String>,
    pub PreventEntryNewStreetsUser: Option<String>,
    pub Conglomerates: Option<String>,
    pub CityID: Option<String>,
    pub SettlementTypeDescriptionRu: Option<String>,
    pub SettlementTypeDescription: Option<String>,
    pub Phone: Option<String>,
    pub TypeOfWarehouse: Option<String>,
    pub Number: Option<String>,
    pub Longitude: Option<String>,
    pub Latitude: Option<String>,
    pub SendingLimitationsOnDimensions: Option<NovaLimitationOnDimensions>,
    pub ReceivingLimitationsOnDimensions: Option<NovaLimitationOnDimensions>,
    pub Reception: Option<NovaSchedule>,
    pub Delivery: Option<NovaSchedule>,
    pub Schedule: Option<NovaSchedule>,
    pub Counterparty: Option<String>,
    pub OwnershipForm: Option<String>,
    pub OwnershipFormDescription: Option<String>,
    pub EDRPOU: Option<String>,
    pub CounterpartyType: Option<String>,
    pub ContactPerson: Option<NovaResponse>,
    pub Redelivery: Option<i32>,
    pub RedeliverySum: Option<i32>,
    pub RedeliveryNum: Option<String>,
    pub RedeliveryPayer: Option<String>,
    pub OwnerDocumentType: Option<String>,
    pub LastCreatedOnTheBasisDocumentType: Option<String>,
    pub LastCreatedOnTheBasisPayerType: Option<String>,
    pub LastCreatedOnTheBasisDateTime: Option<String>,
    pub LastTransactionStatusGM: Option<String>,
    pub LastTransactionDateTimeGM: Option<String>,
    pub DateCreated: Option<String>,
    pub DocumentWeight: Option<f32>,
    pub CheckWeight: Option<f32>,
    pub DocumentCost: Option<String>,
    pub SumBeforeCheckWeight: Option<i32>,
    pub PayerType: Option<String>,
    pub RecipientFullName: Option<String>,
    pub RecipientDateTime: Option<String>,
    pub ScheduledDeliveryDate: Option<String>,
    pub PaymentMethod: Option<String>,
    pub CargoDescriptionString: Option<String>,
    pub CargoType: Option<String>,
    pub CitySender: Option<String>,
    pub CityRecipient: Option<String>,
    pub WarehouseRecipient: Option<String>,
    pub AfterpaymentOnGoodsCost: Option<i32>,
    pub ServiceType: Option<String>,
    pub UndeliveryReasonsSubtypeDescription: Option<String>,
    pub WarehouseRecipientNumber: Option<i32>,
    pub LastCreatedOnTheBasisNumber: Option<String>,
    pub PhoneRecipient: Option<String>,
    pub RecipientFullNameEW: Option<String>,
    pub WarehouseRecipientInternetAddressRef: Option<String>,
    pub MarketplacePartnerToken: Option<String>,
    pub ClientBarcode: Option<String>,
    pub RecipientAddress: Option<String>,
    pub CounterpartyRecipientDescription: Option<String>,
    pub CounterpartySenderType: Option<String>,
    pub DateScan: Option<String>,
    pub PaymentStatus: Option<String>,
    pub PaymentStatusDate: Option<String>,
    pub AmountToPay: Option<String>,
    pub AmountPaid: Option<String>,
    pub Status: Option<String>,
    pub StatusCode: Option<String>,
    pub RefEW: Option<String>,
    // "BackwardDeliverySubTypesServices": [],
    // "BackwardDeliverySubTypesActions": [],
    pub UndeliveryReasons: Option<String>,
    pub Cost: Option<i32>,
    pub CostRedelivery: Option<i32>,
    pub CostOnSite: Option<i32>,
    pub EstimatedDeliveryDate: Option<String>,
    pub IntDocNumber: Option<String>,
    pub TypeDocument: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(default)]
pub struct NovaLimitationOnDimensions {
    pub Width: Option<i32>,
    pub Height: Option<i32>,
    pub Lenght: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NovaSchedule {
    pub Monday: String,
    pub Tuesday: String,
    pub Wednesday: String,
    pub Thursday: String,
    pub Friday: String,
    pub Saturday: String,
    pub Sunday: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct NovaRedeliveryCalculate {
    pub CargoType: String,
    pub Amount: String,
}

#[derive(Debug, Clone)]
pub struct NovaRequestError {
    details: String,
}

impl NovaRequestError {
    #[allow(dead_code)]
    pub fn new(msg: String) -> NovaRequestError {
        NovaRequestError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for NovaRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for NovaRequestError {
    fn description(&self) -> &str {
        &self.details
    }
}
