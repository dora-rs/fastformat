use eyre::{Context, OptionExt, Report, Result};
use std::{collections::HashMap, sync::Arc};

pub struct FastFormatArrowRawData {
    buffers: HashMap<String, arrow::buffer::Buffer>,
    offset_buffers: HashMap<String, arrow::buffer::OffsetBuffer<i32>>,

    array_data: HashMap<String, arrow::array::ArrayData>,
}

#[derive(Default)]
pub struct FastFormatArrowBuilder {
    union_children: Vec<arrow::array::ArrayRef>,
    union_fields: Vec<(i8, arrow::datatypes::FieldRef)>,
}

impl FastFormatArrowRawData {
    pub fn new(array_data: arrow::array::ArrayData) -> Result<Self> {
        use arrow::array::Array;

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

        Ok(Self {
            buffers: HashMap::new(),
            offset_buffers: HashMap::new(),
            array_data: result,
        })
    }

    pub fn load_primitive<T: arrow::datatypes::ArrowPrimitiveType>(
        self,
        field: &str,
    ) -> Result<Self> {
        let mut array_data = self.array_data;
        let mut buffers = self.buffers;
        let offset_buffers = self.offset_buffers;

        let data = array_data.remove(field).ok_or_eyre(Report::msg(format!(
            "Invalid field {} for this map of data",
            field
        )))?;

        let array = arrow::array::PrimitiveArray::<T>::from(data);
        let (_, buffer, _) = array.into_parts();

        buffers.insert(field.to_string(), buffer.into_inner());

        Ok(Self {
            buffers,
            offset_buffers,
            array_data,
        })
    }

    pub fn load_utf(self, field: &str) -> Result<Self> {
        let mut array_data = self.array_data;
        let mut buffers = self.buffers;
        let mut offset_buffers = self.offset_buffers;

        let data = array_data.remove(field).ok_or_eyre(Report::msg(format!(
            "Invalid field {} for this map of data",
            field
        )))?;

        let array = arrow::array::StringArray::from(data);
        let (offset_buffer, buffer, _) = array.into_parts();

        buffers.insert(field.to_string(), buffer);
        offset_buffers.insert(field.to_string(), offset_buffer);

        Ok(Self {
            buffers,
            offset_buffers,
            array_data,
        })
    }

    pub fn utf8_singleton(&self, field: &str) -> Result<String> {
        let buffer = self.buffers.get(field).ok_or_eyre(Report::msg(format!(
            "Invalid field {} for this map of data",
            field
        )))?;

        let offset_buffer = self
            .offset_buffers
            .get(field)
            .ok_or_eyre(Report::msg(format!(
                "Invalid field {} for this map of data",
                field
            )))?;

        let slice = buffer.as_slice();
        let mut iterator = offset_buffer.iter();
        iterator.next();

        let last_offset = iterator.next().cloned().ok_or_eyre(Report::msg(format!(
            "No offset associated with field {}",
            field
        )))? as usize;

        let slice = &slice[0..last_offset];

        String::from_utf8(slice.to_vec()).wrap_err(Report::msg("Invalid UTF-8 string"))
    }

    pub fn utf16_singleton(&self, field: &str) -> Result<String> {
        let buffer = self.buffers.get(field).ok_or_eyre(Report::msg(format!(
            "Invalid field {} for this map of data",
            field
        )))?;

        let offset_buffer = self
            .offset_buffers
            .get(field)
            .ok_or_eyre(Report::msg(format!(
                "Invalid field {} for this map of data",
                field
            )))?;

        let slice = buffer.typed_data::<u16>();
        let mut iterator = offset_buffer.iter();
        iterator.next();

        let last_offset = iterator.next().cloned().ok_or_eyre(Report::msg(format!(
            "No offset associated with field {}",
            field
        )))? as usize;

        let slice = &slice[0..last_offset];

        String::from_utf16(slice).wrap_err(Report::msg("Invalid UTF-16 string"))
    }

    pub fn primitive_singleton<T: arrow::datatypes::ArrowPrimitiveType>(
        &self,
        field: &str,
    ) -> Result<T::Native> {
        let buffer = self.buffers.get(field).ok_or_eyre(Report::msg(format!(
            "Invalid field {} for this map of data",
            field
        )))?;

        let slice = buffer.typed_data::<T::Native>();

        Ok(slice[0])
    }

    pub fn utf8_array(&self, field: &str) -> Result<Vec<String>> {
        let buffer = self.buffers.get(field).ok_or_eyre(Report::msg(format!(
            "Invalid field {} for this map of data",
            field
        )))?;

        let offset_buffer = self
            .offset_buffers
            .get(field)
            .ok_or_eyre(Report::msg(format!(
                "Invalid field {} for this map of data",
                field
            )))?;

        let slice = buffer.as_slice();
        let mut iterator = offset_buffer.iter();
        iterator.next();

        let mut last_offset = 0;

        iterator
            .map(|&offset| {
                let offset = offset as usize;
                let slice = &slice[last_offset..offset];
                last_offset = offset;

                String::from_utf8(slice.to_vec())
                    .wrap_err(Report::msg("Array is not UTF-8 encoded."))
            })
            .collect::<Result<Vec<String>>>()
    }

    pub fn utf16_array(&self, field: &str) -> Result<Vec<String>> {
        let buffer = self.buffers.get(field).ok_or_eyre(Report::msg(format!(
            "Invalid field {} for this map of data",
            field
        )))?;

        let offset_buffer = self
            .offset_buffers
            .get(field)
            .ok_or_eyre(Report::msg(format!(
                "Invalid field {} for this map of data",
                field
            )))?;

        let slice = buffer.typed_data::<u16>();
        let mut iterator = offset_buffer.iter();
        iterator.next();

        let mut last_offset = 0;

        iterator
            .map(|&offset| {
                let offset = offset as usize;
                let slice = &slice[last_offset..offset];
                last_offset = offset;

                String::from_utf16(slice).wrap_err(Report::msg("Array is not UTF-16 encoded."))
            })
            .collect::<Result<Vec<String>>>()
    }

    pub fn primitive_array_view<'a, T: arrow::datatypes::ArrowPrimitiveType>(
        &'a self,
        field: &str,
    ) -> Result<&'a [T::Native]> {
        let buffer = self.buffers.get(field).ok_or_eyre(Report::msg(format!(
            "Invalid field {} for this map of data",
            field
        )))?;

        let slice = buffer.typed_data::<T::Native>();

        Ok(slice)
    }

    pub fn primitive_array<T: arrow::datatypes::ArrowPrimitiveType>(
        &mut self,
        field: &str,
    ) -> Result<Vec<T::Native>> {
        let buffer = self.buffers.remove(field).ok_or_eyre(Report::msg(format!(
            "Invalid field {} for this map of data",
            field
        )))?;

        match buffer.into_vec::<T::Native>() {
            Ok(vec) => Ok(vec),
            Err(buffer) => {
                self.buffers.insert(field.to_string(), buffer);

                Err(Report::msg("Invalid primitive array type. Or the buffer is shared. If you're not sure that the buffer is owned, use primitive_array_view instead."))
            }
        }
    }
}

impl FastFormatArrowBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_primitive_singleton<T: arrow::datatypes::ArrowPrimitiveType>(
        self,
        field: &str,
        value: T::Native,
        data_type: arrow::datatypes::DataType,
        nullable: bool,
    ) -> Self {
        let mut union_children = self.union_children;
        let mut union_fields = self.union_fields;

        let index = union_children.len();

        let data = Arc::new(arrow::array::PrimitiveArray::<T>::from_value(value, 1));
        union_children.push(data);

        let field = (
            index as i8,
            Arc::new(arrow::datatypes::Field::new(field, data_type, nullable)),
        );
        union_fields.push(field);

        Self {
            union_children,
            union_fields,
        }
    }

    pub fn push_primitive_array<T: arrow::datatypes::ArrowPrimitiveType>(
        self,
        field: &str,
        value: Vec<T::Native>,
        data_type: arrow::datatypes::DataType,
        nullable: bool,
    ) -> Self {
        let mut union_children = self.union_children;
        let mut union_fields = self.union_fields;

        let index = union_children.len();

        let data = Arc::new(arrow::array::PrimitiveArray::<T>::from_iter_values(value));
        union_children.push(data);

        let field = (
            index as i8,
            Arc::new(arrow::datatypes::Field::new(field, data_type, nullable)),
        );
        union_fields.push(field);

        Self {
            union_children,
            union_fields,
        }
    }

    pub fn push_utf_singleton(
        self,
        field: &str,
        value: String,
        data_type: arrow::datatypes::DataType,
        nullable: bool,
    ) -> Self {
        let mut union_children = self.union_children;
        let mut union_fields = self.union_fields;

        let index = union_children.len();

        let data = Arc::new(arrow::array::StringArray::from(vec![value]));
        union_children.push(data);

        let field = (
            index as i8,
            Arc::new(arrow::datatypes::Field::new(field, data_type, nullable)),
        );
        union_fields.push(field);

        Self {
            union_children,
            union_fields,
        }
    }

    pub fn push_utf_array(
        self,
        field: &str,
        value: Vec<String>,
        data_type: arrow::datatypes::DataType,
        nullable: bool,
    ) -> Self {
        let mut union_children = self.union_children;
        let mut union_fields = self.union_fields;

        let index = union_children.len();

        let data = Arc::new(arrow::array::StringArray::from(value));
        union_children.push(data);

        let field = (
            index as i8,
            Arc::new(arrow::datatypes::Field::new(field, data_type, nullable)),
        );
        union_fields.push(field);

        Self {
            union_children,
            union_fields,
        }
    }

    pub fn into_arrow(self) -> Result<arrow::array::ArrayData> {
        use arrow::array::Array;

        let type_ids = [].into_iter().collect::<arrow::buffer::ScalarBuffer<i8>>();
        let offsets = [].into_iter().collect::<arrow::buffer::ScalarBuffer<i32>>();

        let union_fields = self
            .union_fields
            .into_iter()
            .collect::<arrow::datatypes::UnionFields>();

        Ok(arrow::array::UnionArray::try_new(
            union_fields,
            type_ids,
            Some(offsets),
            self.union_children,
        )
        .wrap_err("Failed to create UnionArray with Image data.")?
        .into_data())
    }
}
