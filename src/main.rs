use std::{
    collections::{btree_map::Entry, BTreeMap},
    path::Path,
};

use clap::{app_from_crate, arg};
use convert_case::Case;
use regex::Regex;
use serde::de::DeserializeOwned;
use serde_json::Value;

fn main() {
    let matches = app_from_crate!()
        .arg(
            arg!([path] "JSON file path to analyze")
                .required(true)
                .allow_invalid_utf8(true),
        )
        .get_matches();

    let path = Path::new(matches.value_of_os("path").unwrap());

    analyze(path);
}

fn analyze<P: AsRef<Path>>(path: P) {
    let root: Value = read_json_file(path).unwrap();

    let root_type = value_to_type(&root);

    print_object_types("root", &root_type);
}

fn print_object_types(struct_ident: &str, t: &UnionType) {
    if !t.object_type.0.is_empty() {
        print_object_type(struct_ident, &t.object_type);
    }
    for (k, t) in &t.object_type.0 {
        let key_ident = convert_case::Casing::to_case(&Regex::new(r"[-/.]").unwrap().replace_all(k, "_"), Case::Snake);
        let struct_ident = format!("{}__{}", struct_ident, key_ident);
        print_object_types(&struct_ident, t);
    }
    for t in t.array_type.as_deref() {
        print_object_types(struct_ident, t)
    }
}

#[derive(Debug, Default)]
struct ObjectType(BTreeMap<String, UnionType>);

impl ObjectType {
    fn merge(&mut self, other: ObjectType) {
        for (key, value) in self.0.iter_mut() {
            if !other.0.contains_key(key) {
                value.is_null = true;
            }
        }
        for (key, value) in other.0.into_iter() {
            match self.0.entry(key) {
                Entry::Occupied(mut occupied) => {
                    occupied.get_mut().merge(value);
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(value);
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct UnionType {
    is_null: bool,
    is_bool: bool,
    is_number: bool,
    is_string: bool,
    object_type: ObjectType,
    array_type: Option<Box<UnionType>>,
}

impl UnionType {
    fn merge(&mut self, other: UnionType) {
        self.is_null |= other.is_null;
        self.is_bool |= other.is_bool;
        self.is_number |= other.is_number;
        self.is_string |= other.is_string;
        self.object_type.merge(other.object_type);
        if let (Some(a), Some(b)) = (self.array_type.as_deref_mut(), other.array_type) {
            a.merge(*b);
        }
    }

    fn null() -> Self {
        Self {
            is_null: true,
            ..Default::default()
        }
    }

    fn bool() -> Self {
        Self {
            is_bool: true,
            ..Default::default()
        }
    }

    fn number() -> Self {
        Self {
            is_number: true,
            ..Default::default()
        }
    }

    fn string() -> Self {
        Self {
            is_string: true,
            ..Default::default()
        }
    }

    fn object(object_type: ObjectType) -> Self {
        Self {
            object_type,
            ..Default::default()
        }
    }

    fn array(array_type: UnionType) -> Self {
        Self {
            array_type: Some(Box::new(array_type)),
            ..Default::default()
        }
    }
}

fn value_to_type(value: &Value) -> UnionType {
    match value {
        Value::Null => UnionType::null(),
        Value::Bool(_) => UnionType::bool(),
        Value::Number(_) => UnionType::number(),
        Value::String(_) => UnionType::string(),
        Value::Object(object) => UnionType::object(ObjectType(
            object
                .iter()
                .map(|(key, value)| (key.clone(), value_to_type(value)))
                .collect(),
        )),
        Value::Array(array) => {
            UnionType::array(array.iter().fold(UnionType::default(), |mut state, value| {
                state.merge(value_to_type(value));
                state
            }))
        }
    }
}

fn union_type_string(p: &str, t: &UnionType) -> String {
    let mut u = Vec::<String>::new();
    if t.is_bool {
        u.push("bool".to_string());
    }
    if t.is_number {
        u.push("f64".to_string());
    }
    if t.is_string {
        u.push("String".to_string());
    }
    if !t.object_type.0.is_empty() {
        u.push(p.to_string())
    }
    if let Some(array_type) = t.array_type.as_deref() {
        u.push(format!("Vec<{}>", union_type_string(p, array_type)));
    }
    let u = u.join(" | ");
    if t.is_null {
        format!("Option<{}>", u)
    } else {
        u
    }
}

fn print_object_type(struct_ident: &str, object_type: &ObjectType) {
    println!("#[derive(Debug, Serialize, Deserialize)]");
    println!("struct {} {{", struct_ident);
    for (k, t) in &object_type.0 {
        let key_ident = convert_case::Casing::to_case(&Regex::new(r"[-/.]").unwrap().replace_all(k, "_"), Case::Snake);
        let struct_ident = format!("{}__{}", struct_ident, key_ident);
        println!("  #[serde(rename = \"{}\")]", k);
        println!("  {}: {},", key_ident, union_type_string(&struct_ident, t));
    }
    println!("}}");
    println!();
}

fn read_json_file<T: DeserializeOwned, P: AsRef<Path>>(
    path: P,
) -> Result<T, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    Ok(serde_json::from_reader(reader)?)
}
