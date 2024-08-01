use std::collections::HashMap;

pub fn look_up_table(fields: &arrow::datatypes::UnionFields) -> HashMap<String, i8> {
    let mut result = HashMap::new();

    for field in fields.iter() {
        let (a, b) = field;

        result.insert(b.name().to_string(), a);
    }

    return result;
}

pub fn retrieve_child<'a, T: 'static>(
    array: &'a arrow::array::UnionArray,
    field: String,
    look_up_table: &'a HashMap<String, i8>,
) -> &'a T {
    let index = look_up_table.get(&field).unwrap().clone();

    return array.child(index).as_any().downcast_ref::<T>().unwrap();
}
