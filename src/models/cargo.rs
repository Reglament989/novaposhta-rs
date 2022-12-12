use core::fmt;

use chrono::{DateTime, Datelike, TimeZone};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Cargo {
    pub cost: i32,
    pub options_seat: NovaOptionsSeat,
    pub payment_on_delivery: bool,
    pub description: String,
}

impl Cargo {
    pub fn new(cost: i32, options: NovaOptionsSeat, payment: bool, description: String) -> Self {
        Cargo {
            cost,
            options_seat: options,
            payment_on_delivery: payment,
            description,
        }
    }
}
pub trait CargoSplit {
    fn into_ttn_values(&self) -> (f32, i32, i32, String);
}

impl CargoSplit for Vec<Cargo> {
    fn into_ttn_values(&self) -> (f32, i32, i32, String) {
        let mut total = 0f32;
        let mut price = 0;
        let mut to_payment_ammount = 0;
        let mut description: String = "".to_string();
        for c in self.into_iter() {
            total += c.options_seat.weight;
            price += c.cost;
            description = c.description.clone();
            if c.payment_on_delivery {
                to_payment_ammount += c.cost;
            }
        }
        (total, price, to_payment_ammount, description)
    }
}

pub trait NovaTime {
    fn into_ttn_time(&self) -> String;
}

impl<Tz: TimeZone> NovaTime for DateTime<Tz> {
    fn into_ttn_time(&self) -> String {
        format!("{}.{}.{}", self.day(), self.month(), self.year())
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug, Clone)]
#[serde(default)]
pub struct NovaOptionsSeat {
    pub Volume: f32,
    pub Width: i32,
    pub Length: i32,
    pub Height: i32,
    pub weight: f32,
}

impl NovaOptionsSeat {
    pub fn new(volume: f32, width: i32, lenght: i32, height: i32, weight: f32) -> Self {
        NovaOptionsSeat {
            Volume: volume,
            Width: width,
            Length: lenght,
            Height: height,
            weight,
        }
    }
}

pub enum NovaParcelWightStandarts {
    UpToHalfKilogram,
}

impl From<NovaParcelWightStandarts> for NovaOptionsSeat {
    fn from(s: NovaParcelWightStandarts) -> Self {
        match s {
            NovaParcelWightStandarts::UpToHalfKilogram => NovaOptionsSeat {
                Volume: 0.5,
                Width: 20,
                Length: 20,
                Height: 5,
                weight: 0.5,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    }
}

impl fmt::Display for NovaServiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
