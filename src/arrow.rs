use std::collections::HashMap;

use eyre::{Context, OptionExt, Report, Result};

/// Converts an Arrow `UnionArray` into a `HashMap`.
///
/// This function takes an Arrow `UnionArray` and converts it into a `HashMap` where the keys
/// are the names of the fields and the values are the corresponding `ArrayRef` objects.
///
/// # Arguments
///
/// * `array` - An `arrow::array::UnionArray` containing the data to be converted.
///
/// # Returns
///
/// A `Result` containing the constructed `HashMap<String, arrow::array::ArrayRef>` if successful,
/// or an error otherwise.
///
/// # Errors
///
/// Returns an error if the union array field index is invalid or if there are issues
/// in accessing the children of the union array.
pub fn arrow_union_into_map(
    array: arrow::array::UnionArray,
) -> Result<HashMap<String, arrow::array::ArrayRef>> {
    let mut result = HashMap::new();

    let (union_fields, _, _, children) = array.into_parts();

    for (a, b) in union_fields.iter() {
        let child = children
            .get(a as usize)
            .ok_or_eyre(Report::msg(
                "Invalid union array field index. Must be >= 0 and correspond to children index in the array.",
            ))?
            .clone();

        result.insert(b.name().to_string(), child);
    }

    Ok(result)
}

/// Extracts a primitive array from a `HashMap` and converts it to a `Vec`.
///
/// This function takes a `HashMap` containing `ArrayRef` objects, extracts the array corresponding
/// to the specified field, and converts it into a `Vec` of the primitive type `T`.
///
/// # Arguments
///
/// * `field` - A string slice representing the key in the `HashMap`.
/// * `map` - A mutable reference to the `HashMap<String, arrow::array::ArrayRef>`.
///
/// # Returns
///
/// A `Result` containing the constructed `Vec<T>` if successful, or an error otherwise.
///
/// # Type Parameters
///
/// * `T` - The native primitive type to be extracted.
/// * `G` - The Arrow primitive type that corresponds to `T`.
///
/// # Errors
///
/// Returns an error if the specified field is not present in the `HashMap`, or if the type
/// conversion fails.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use arrow::array::{Int32Array, ArrayRef};
/// use std::sync::Arc;
///
/// use fastformat::arrow::get_primitive_array_from_map;
///
/// let mut map = HashMap::new();
/// map.insert("field".to_string(), Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef);
///
/// let result: Vec<i32> = get_primitive_array_from_map::<i32, arrow::datatypes::Int32Type>("field", &mut map).unwrap();
/// ```
pub fn get_primitive_array_from_map<
    T: arrow::datatypes::ArrowNativeType,
    G: arrow::datatypes::ArrowPrimitiveType,
>(
    field: &str,
    map: &mut HashMap<String, arrow::array::ArrayRef>,
) -> Result<Vec<T>> {
    use arrow::array::Array;

    let array_data = map
        .remove(field)
        .ok_or_eyre(Report::msg("Invalid field for this map."))?
        .into_data();

    let array = arrow::array::PrimitiveArray::<G>::from(array_data);
    let (_, buffer, _) = array.into_parts();
    let buffer = buffer.into_inner();

    match buffer.into_vec::<T>() {
        Ok(vec) => Ok(vec),
        Err(e) => Err(Report::msg(format!(
            "T is not a valid type for this buffer. Must have the same layout as the buffer (it usually occurs when type is incorrect or when an other reference exists). Error: {:?}", e
        ))),
    }
}

/// Extracts a UTF-8 encoded string array from a `HashMap` and converts it to a `Vec<String>`.
///
/// This function takes a `HashMap` containing `ArrayRef` objects, extracts the UTF-8 encoded
/// string array corresponding to the specified field, and converts it into a `Vec<String>`.
///
/// # Arguments
///
/// * `field` - A string slice representing the key in the `HashMap`.
/// * `map` - A mutable reference to the `HashMap<String, arrow::array::ArrayRef>`.
///
/// # Returns
///
/// A `Result` containing the constructed `Vec<String>` if successful, or an error otherwise.
///
/// # Errors
///
/// Returns an error if the specified field is not present in the `HashMap`, or if the array
/// is not UTF-8 encoded.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use arrow::array::{StringArray, ArrayRef};
/// use std::sync::Arc;
///
/// use fastformat::arrow::get_utf8_array_from_map;
///
/// let mut map = HashMap::new();
/// map.insert("field".to_string(), Arc::new(StringArray::from(vec!["a", "b", "c"])) as ArrayRef);
///
/// let result = get_utf8_array_from_map("field", &mut map).unwrap();
/// ```
pub fn get_utf8_array_from_map(
    field: &str,
    map: &mut HashMap<String, arrow::array::ArrayRef>,
) -> Result<Vec<String>> {
    use arrow::array::Array;

    let array_data = map
        .remove(field)
        .ok_or_eyre(Report::msg("Invalid field for this map."))?
        .into_data();

    let array = arrow::array::StringArray::from(array_data);
    let (offsets, buffer, _) = array.into_parts();

    let slice = buffer.as_slice();
    let mut last_offset = 0;
    let mut iterator = offsets.iter();
    iterator.next();

    iterator
        .map(|&offset| {
            let offset = offset as usize;
            let slice = &slice[last_offset..offset];
            last_offset = offset;

            String::from_utf8(slice.to_vec()).wrap_err(Report::msg("Array is not UTF-8 encoded."))
        })
        .collect::<Result<Vec<String>>>()
}
