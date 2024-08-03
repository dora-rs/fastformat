use arrow::datatypes::DataType;

use std::{collections::HashMap, sync::Arc};

use eyre::{ContextCompat, Result};

/// Creates a lookup table (`HashMap`) from the fields of a union.
///
/// This function takes a reference to `arrow::datatypes::UnionFields` and
/// creates a `HashMap` where the field names are the keys (as `String`) and
/// the associated values are the field identifiers (`i8`).
///
/// # Arguments
///
/// * `fields` - A reference to the fields of the Arrow union data structure (`arrow::datatypes::UnionFields`).
///
/// # Returns
///
/// A `HashMap` with the field names as keys and their identifiers (`i8`) as values.
///
/// # Example
///
/// ```
/// use arrow::datatypes::{Field, DataType, UnionFields};
/// use std::collections::HashMap;
///
/// use fastformat::arrow::union_lookup_table;
///
/// let fields = UnionFields::new(
///     vec![1, 2],
///     vec![
///         Field::new("field1", DataType::Int32, false),
///         Field::new("field2", DataType::Float64, false),
///     ],
/// );
///
/// let lookup_table = union_lookup_table(&fields);
///
/// assert_eq!(lookup_table.get("field1"), Some(&1));
/// assert_eq!(lookup_table.get("field2"), Some(&2));
/// ```
pub fn union_lookup_table(fields: &arrow::datatypes::UnionFields) -> HashMap<String, i8> {
    let mut result = HashMap::new();

    for field in fields.iter() {
        let (a, b) = field;

        result.insert(b.name().to_string(), a);
    }

    return result;
}

/// Retrieves a column from a `UnionArray` by its field name and downcasts it to the specified type.
///
/// This function takes a reference to an `arrow::array::UnionArray`, a field name,
/// and a lookup table mapping field names to their identifiers. It retrieves the column
/// corresponding to the field name from the union array and attempts to downcast it to
/// the specified type `T`.
///
/// # Arguments
///
/// * `array` - A reference to the `UnionArray` from which to retrieve the column.
/// * `field` - The name of the field whose column is to be retrieved.
/// * `lookup_table` - A reference to a `HashMap` that maps field names (`String`) to their identifiers (`i8`).
///
/// # Returns
///
/// A `Result` containing a reference to the column cast to type `T` if successful, or an error otherwise.
///
/// # Errors
///
/// Returns an error if the field name is not found in the lookup table, or if the retrieved column
/// cannot be downcast to the specified type `T`.
///
/// # Example
///
/// ```
/// use arrow::array::Array;
///
/// use fastformat::image::Image;
/// use fastformat::arrow::union_lookup_table;
/// use fastformat::arrow::column_by_name;
///
/// let pixels = vec![0; 27]; // 3x3 image with 3 bytes per pixel
/// let image = Image::new_bgr8(pixels, 3, 3, None).unwrap();
/// let array = image.to_arrow().unwrap();
///
/// let union_fields = match array.data_type() {
///    arrow::datatypes::DataType::Union(fields, ..) => fields,
///    _ => panic!("Unexpected data type for image array")
/// };
///
/// let lookup_table = union_lookup_table(&union_fields);
///
/// let int_column = column_by_name::<arrow::array::Int32Array>(&array, "field1", &lookup_table);
/// ```
pub fn column_by_name<'a, T: 'static>(
    array: &'a arrow::array::UnionArray,
    field: &'a str,
    lookup_table: &'a HashMap<String, i8>,
) -> Result<&'a T> {
    let index = lookup_table
        .get(field)
        .cloned()
        .wrap_err(format!("Couldn't get field {} from look_up table", field))?;

    return array
        .child(index)
        .as_any()
        .downcast_ref::<T>()
        .wrap_err(format!("Couldn't downcast field {} to type T", field));
}

/// Creates a tuple representing a union field with an index and an `Arc`-wrapped `Field`.
///
/// This function constructs a tuple where the first element is the given index and the second element
/// is an `Arc`-wrapped `Field` constructed using the provided name, data type, and nullability.
///
/// # Arguments
///
/// * `index` - An identifier (`i8`) for the union field.
/// * `name` - A string slice representing the name of the field.
/// * `data_type` - The data type of the field (`arrow::datatypes::DataType`).
/// * `nullable` - A boolean indicating whether the field is nullable.
///
/// # Returns
///
/// A tuple where the first element is the given index (`i8`) and the second element is an `Arc`-wrapped
/// `Field` constructed from the provided name, data type, and nullability.
///
/// # Example
///
/// ```
/// use arrow::datatypes::{DataType, Field};
/// use std::sync::Arc;
/// use fastformat::arrow::union_field;
///
/// let index = 1;
/// let name = "field1";
/// let data_type = DataType::Int32;
/// let nullable = false;
///
/// let union_field_tuple = union_field(index, name, data_type, nullable);
///
/// assert_eq!(union_field_tuple.0, 1);
/// assert_eq!(union_field_tuple.1.name(), "field1");
/// assert_eq!(union_field_tuple.1.data_type(), &DataType::Int32);
/// assert_eq!(union_field_tuple.1.is_nullable(), false);
/// ```
///
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
