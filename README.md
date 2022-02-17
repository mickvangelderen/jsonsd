Generate a JSON schema implemented in Rust based on example JSON data.

For example, given `input.json`:

```json
{
    "data": [
        {
            "name": "Strawberry",
            "price": 100
        },
        {
            "name": "Banana"
        }
    ]
}
```

Calling `jsonsd input.json` will output something like:

```rust
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
  price: Option<f64>,
}
```

See [the assets](./assets/) for other examples.

## Opportunities

 * somehow generate valid rust code to deal with type unions like `Number | String`. 
 * be smarter about json schemas where the field keys are dynamic or in other words the field keys are based on the data that is represented.
 * allow outputting a JSON schema.
 * build an interactive UI around this idea where the user is asked to review deeper and deeper objects, nullability can be toggled, ...
