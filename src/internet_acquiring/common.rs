use base64::{Engine, engine::general_purpose};
use serde::Serialize;

#[derive(Serialize, Default, Debug)]
pub struct DetailAddenda {
    #[serde(rename = "airLine", skip_serializing_if = "Option::is_none")]
    airline: Option<String>,
    #[serde(rename = "ticketNumber", skip_serializing_if = "Option::is_none")]
    ticket_number: Option<String>,
    #[serde(rename = "passengerName", skip_serializing_if = "Option::is_none")]
    passenger_name: Option<String>,
    #[serde(rename = "flightNumber", skip_serializing_if = "Option::is_none")]
    flight_number: Option<String>,
    #[serde(rename = "originCity", skip_serializing_if = "Option::is_none")]
    origin_city: Option<String>,
    #[serde(rename = "destinationCity", skip_serializing_if = "Option::is_none")]
    destination_city: Option<String>,
    #[serde(rename = "departureDate", skip_serializing_if = "Option::is_none")]
    departure_date: Option<u32>,
}

impl DetailAddenda {
    pub fn airline(mut self, airline: String) -> Self {
        self.airline = Some(airline);
        self
    }

    pub fn ticket_number(mut self, ticket_number: String) -> Self {
        self.ticket_number = Some(ticket_number);
        self
    }

    pub fn passenger_name(mut self, passenger_name: String) -> Self {
        self.passenger_name = Some(passenger_name);
        self
    }

    pub fn flight_number(mut self, flight_number: String) -> Self {
        self.flight_number = Some(flight_number);
        self
    }

    pub fn origin_city(mut self, origin_city: String) -> Self {
        self.origin_city = Some(origin_city);
        self
    }

    pub fn destination_city(mut self, destination_city: String) -> Self {
        self.destination_city = Some(destination_city);
        self
    }

    pub fn departure_date(mut self, departure_date: u32) -> Self {
        self.departure_date = Some(departure_date);
        self
    }

    pub fn to_base64(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap_or(String::new());
        let base64 = general_purpose::STANDARD.encode(serialized);

        base64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Item {
    id: u32,
    amount: u32,
    cost: f64,
    price: f64,
}

impl Item {
    pub fn new(id: u32, amount: u32, cost: f64, price: f64) -> Self {
        Self {
            id,
            amount,
            cost,
            price,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RroInfo {
    items: Option<Vec<Item>>,
    delivery_emails: Option<Vec<String>>,
}

impl RroInfo {
    pub fn new() -> Self {
        Self {
            items: None,
            delivery_emails: None,
        }
    }

    pub fn items(mut self, items: Vec<Item>) -> Self {
        self.items = Some(items);
        self
    }

    pub fn emails(mut self, emails: Vec<String>) -> Self {
        self.delivery_emails = Some(emails);
        self
    }
}
