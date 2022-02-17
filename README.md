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

 * generate valid rust code for type unions `Number | String`. 
 * allow annotating or automatically recognize objects being used as dynamic maps meaning the field names depend on the input data.
 * allow annotating or automatically recognize enumeration variants and their discriminants.
 * allow outputting a JSON schema instead of rust code.
 * allow controlling whether a `#[serde(rename = "...")]` attribute should be emitted when the field name and rust identifier are exactly the same.
 * build an interactive UI around this idea where the user is asked to review deeper and deeper objects, nullability can be toggled, ...
