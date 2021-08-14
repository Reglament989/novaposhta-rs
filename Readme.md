# Novaposhta-rs
This a small implementation of novaposhta http api
with small **abstraction**. If you wont use my abstraction you can use instead NovaposhtaRaw with full identityfy with their documentation
*but their documentation soo shity because outdated and not entirely true.*

# Usage
### Create ttn with backward delivery.
```rust
let nova = Novaposhta::default();
let date_time = {
    let now = Local::today() + Duration::days(1);
    format!("{}.{}.{}", now.day(), now.month(), now.year())
};
let payload = CreateNewTtnPayload::new(
    Recipient::new(
        "Киев",
        "имя",
        Some("отчество"),
        "фамилия",
        "номер телефона",
        true, // is_payer
        Address::warehouse(14),
    ),
    Sender::new("Киев", "отделение", "номер телефона"),
    date_time, // Time of
    NovaPaymentMethod::Cash,
    "Аксесуары".to_string(),
    vec![Cargo::new(150, 0.5, None)],
    "Parcel".to_string(),
    Some(BackwardDeliveryData::money(150)),
);
/* CreateNewTtnResponse */ nova.create_ttn(payload).await?;
```
```rust
Novaposhta::default(); // take api_key from env "NOVAPOSHTA_KEY"
Novaposhta::new("API_KEY"); // if you cannot use env
```

### Delete ttns
```rust
let nova = Novaposhta::default();
let payload = vec![
    "TTN ref".to_string()
];
/* Vec<String> */ nova.delete_ttn(payload).await?; 
// Always check if your document in delete list from response
```
 
### Track ttns
```rust
let nova = Novaposhta::default();
let payload = vec![
    NovaDocument::new("Number ttn".to_string(), "Phone".to_string()),
    // Sender or Recipient
];
/* Vec<NovaResponseData> */ nova.track_ttn(payload).await?;
```
For more info and available fields refer to [novaposhta](https://devcenter.novaposhta.ua/docs/services/556eef34a0fe4f02049c664e/operations/55702cbba0fe4f0cf4fc53ee).

### Disclaimer
This abstraction not so fast because we make 4 requests for simple create ttn...
This can be fixed, but then there will be no point in abstractions. If you want to use this api as it was conceived in a novaposhta, please pay attention for [NovaposhtaRaw](https://tokyjo.gitlab.io/novaposhta-rs/novaposhta/raw/struct.NovaposhtaRaw.html)
#### This is my first project on rust. pull requests and comments welcome in issues