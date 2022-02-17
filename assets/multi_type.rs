#[derive(Debug, Serialize, Deserialize)]
struct root {
  #[serde(rename = "data")]
  data: Vec<root__data>,
}

#[derive(Debug, Serialize, Deserialize)]
struct root__data {
  #[serde(rename = "name")]
  name: String,
  #[serde(rename = "price")]
  price: f64 | String,
}

