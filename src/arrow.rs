use arrow::datatypes::DataType;

use std::{collections::HashMap, sync::Arc};

use eyre::{ContextCompat, Result};

pub fn union_look_up_table(fields: &arrow::datatypes::UnionFields) -> HashMap<String, i8> {
    let mut result = HashMap::new();

    for field in fields.iter() {
        let (a, b) = field;

        result.insert(b.name().to_string(), a);
    }

    return result;
}

pub fn column_by_name<'a, T: 'static>(
    array: &'a arrow::array::UnionArray,
    field: &'a str,
    look_up_table: &'a HashMap<String, i8>,
) -> Result<&'a T> {
    let index = look_up_table
        .get(field)
        .cloned()
        .wrap_err(format!("Couldn't get field {} from look_up table", field))?;

    return array
        .child(index)
        .as_any()
        .downcast_ref::<T>()
        .wrap_err(format!("Couldn't downcast field {} to type T", field));
}

pub fn union_field(
    index: i8,
    name: &str,
    data_type: DataType,
    nullable: bool,
) -> (i8, Arc<arrow::datatypes::Field>) {
    (
        index,
        Arc::new(arrow::datatypes::Field::new(name, data_type, nullable)),
    )
}
