use arrow::array::Array;
use eyre::{Context, OptionExt, Report, Result};
use std::collections::HashMap;

pub fn array_data_to_map(
    array_data: arrow::array::ArrayData,
) -> Result<HashMap<String, arrow::array::ArrayData>> {
    let array = arrow::array::UnionArray::from(array_data);

    let mut result = HashMap::new();

    let (union_fields, _, _, children) = array.into_parts();

    for (a, b) in union_fields.iter() {
        let child = children
            .get(a as usize)
            .ok_or_eyre(Report::msg(
                format!(
                    "Invalid union array field {}'s index (= {}). Must be >= 0 and correspond to children index in the array",
                    b, a
                ),
            ))?
            .clone()
            .into_data();

        result.insert(b.name().to_string(), child);
    }

    Ok(result)
}

pub fn primitive_buffer_from_map<T: arrow::datatypes::ArrowPrimitiveType>(
    field: &str,
    map: &mut HashMap<String, arrow::array::ArrayData>,
) -> Result<(
    arrow::buffer::Buffer,
    Option<arrow::buffer::OffsetBuffer<i32>>,
)> {
    let array_data = map.remove(field).ok_or_eyre(Report::msg(format!(
        "Invalid field {} for this map of data",
        field
    )))?;

    let array = arrow::array::PrimitiveArray::<T>::from(array_data);
    let (_, buffer, _) = array.into_parts();

    Ok((buffer.into_inner(), None))
}

pub fn utf8_buffer_from_map(
    field: &str,
    map: &mut HashMap<String, arrow::array::ArrayData>,
) -> Result<(
    arrow::buffer::Buffer,
    Option<arrow::buffer::OffsetBuffer<i32>>,
)> {
    let array_data = map.remove(field).ok_or_eyre(Report::msg(format!(
        "Invalid field {} for this map of data",
        field
    )))?;

    let array = arrow::array::StringArray::from(array_data);
    let (offset, buffer, _) = array.into_parts();

    Ok((buffer, Some(offset)))
}

pub fn primitive_singleton_from_raw_parts<T: arrow::datatypes::ArrowPrimitiveType>(
    field: &str,
    raw_parts: &HashMap<
        String,
        (
            arrow::buffer::Buffer,
            Option<arrow::buffer::OffsetBuffer<i32>>,
        ),
    >,
) -> Result<T::Native> {
    let (buffer, _) = raw_parts.get(field).ok_or_eyre(Report::msg(format!(
        "Invalid field {} for this map of data",
        field
    )))?;

    let slice = buffer.typed_data::<T::Native>();

    slice
        .first()
        .cloned()
        .ok_or_eyre(Report::msg(format!("Field {} is empty", field)))
}

pub fn utf8_singleton_from_raw_parts(
    field: &str,
    raw_parts: &HashMap<
        String,
        (
            arrow::buffer::Buffer,
            Option<arrow::buffer::OffsetBuffer<i32>>,
        ),
    >,
) -> Result<String> {
    let (buffer, offset_buffer) = raw_parts.get(field).ok_or_eyre(Report::msg(format!(
        "Invalid field {} for this map of data",
        field
    )))?;

    let offset = offset_buffer.clone().ok_or_eyre(Report::msg(format!(
        "No offset associated with field {}",
        field
    )))?;

    let slice = buffer.as_slice();
    let mut iterator = offset.iter();
    iterator.next();

    let last_offset = iterator.next().cloned().ok_or_eyre(Report::msg(format!(
        "No offset associated with field {}",
        field
    )))? as usize;

    let slice = &slice[0..last_offset];

    String::from_utf8(slice.to_vec()).wrap_err(Report::msg("Invalid UTF-8 string"))
}

pub fn primitive_array_from_raw_parts<T: arrow::datatypes::ArrowPrimitiveType>(
    field: &str,
    raw_parts: &mut HashMap<
        String,
        (
            arrow::buffer::Buffer,
            Option<arrow::buffer::OffsetBuffer<i32>>,
        ),
    >,
) -> Result<Vec<T::Native>> {
    let (buffer, _) = raw_parts.remove(field).ok_or_eyre(Report::msg(format!(
        "Invalid field {} for this map.",
        field
    )))?;

    match buffer.into_vec::<T::Native>() {
        Ok(vec) => Ok(vec),
        Err(_) => Err(Report::msg("Invalid type for this buffer")),
    }
}

pub fn primitive_array_view_from_raw_parts<'a, T: arrow::datatypes::ArrowPrimitiveType>(
    field: &str,
    raw_parts: &'a HashMap<
        String,
        (
            arrow::buffer::Buffer,
            Option<arrow::buffer::OffsetBuffer<i32>>,
        ),
    >,
) -> Result<&'a [T::Native]> {
    let (buffer, _) = raw_parts.get(field).ok_or_eyre(Report::msg(format!(
        "Invalid field {} for this map.",
        field
    )))?;

    Ok(buffer.typed_data::<T::Native>())
}

pub fn utf8_array_from_raw_parts(
    field: &str,
    raw_parts: &mut HashMap<
        String,
        (
            arrow::buffer::Buffer,
            Option<arrow::buffer::OffsetBuffer<i32>>,
        ),
    >,
) -> Result<Vec<String>> {
    let (buffer, offset_buffer) = raw_parts.remove(field).ok_or_eyre(Report::msg(format!(
        "Invalid field {} for this map of data",
        field
    )))?;

    let offset = offset_buffer.ok_or_eyre(Report::msg(format!(
        "No offset associated with field {}",
        field
    )))?;

    let slice = buffer.as_slice();
    let mut iterator = offset.iter();
    iterator.next();

    let mut last_offset = 0;

    iterator
        .map(|&offset| {
            let offset = offset as usize;
            let slice = &slice[last_offset..offset];
            last_offset = offset;

            String::from_utf8(slice.to_vec()).wrap_err(Report::msg("Array is not UTF-8 encoded."))
        })
        .collect::<Result<Vec<String>>>()
}
