use std::collections::HashMap;

use eyre::OptionExt;

pub struct ArrowDataViewer {
    array_data: HashMap<String, arrow::array::ArrayData>,

    buffers: HashMap<String, arrow::buffer::Buffer>,
    offset_buffers: HashMap<String, arrow::buffer::OffsetBuffer<i32>>,
}

impl ArrowDataViewer {
    pub fn new(array_data: arrow::array::ArrayData) -> eyre::Result<Self> {
        use arrow::array::Array;

        let array = arrow::array::UnionArray::from(array_data);

        let mut result = HashMap::new();

        let (union_fields, _, _, children) = array.into_parts();

        for (a, b) in union_fields.iter() {
            let child = children
                .get(a as usize)
                .ok_or_eyre(eyre::eyre!(
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
            array_data: result,
            buffers: HashMap::new(),
            offset_buffers: HashMap::new(),
        })
    }

    pub fn primitive_singleton<T: arrow::datatypes::ArrowPrimitiveType>(
        &self,
        field: &str,
    ) -> eyre::Result<T::Native> {
        let data = self.array_data.get(field).ok_or_eyre(eyre::eyre!(format!(
            "Invalid field {} for this map of data",
            field
        )))?;

        let array = arrow::array::PrimitiveArray::<T>::from(data.clone());
        let (_, buffer, _) = array.into_parts();

        let inner = buffer.into_inner();

        let slice = inner.typed_data::<T::Native>();

        slice.first().cloned().ok_or_eyre(eyre::eyre!(format!(
            "Failed to get the first element of the buffer for field {}",
            field
        )))
    }

    pub fn utf8_singleton(&self, field: &str) -> eyre::Result<String> {
        let data = self.array_data.get(field).ok_or_eyre(eyre::eyre!(format!(
            "Invalid field {} for this map of data",
            field
        )))?;

        let array = arrow::array::StringArray::from(data.clone());
        let (offset_buffer, buffer, _) = array.into_parts();

        let slice = buffer.as_slice();
        let mut iterator = offset_buffer.iter();
        iterator.next();

        let last_offset = iterator.next().cloned().ok_or_eyre(eyre::eyre!(format!(
            "No offset associated with field {}",
            field
        )))? as usize;

        let slice = &slice[0..last_offset];

        String::from_utf8(slice.to_vec()).map_err(|e| eyre::eyre!(e))
    }

    pub fn load_primitive<T: arrow::datatypes::ArrowPrimitiveType>(
        self,
        field: &str,
    ) -> eyre::Result<Self> {
        let mut buffers = self.buffers;
        let mut array_data = self.array_data;

        let data = array_data.remove(field).ok_or_eyre(eyre::eyre!(format!(
            "Invalid field {} for this map of data",
            field
        )))?;

        let array = arrow::array::PrimitiveArray::<T>::from(data);
        let (_, buffer, _) = array.into_parts();

        let inner = buffer.into_inner();

        buffers.insert(field.to_string(), inner);

        Ok(Self {
            array_data,
            buffers,
            offset_buffers: self.offset_buffers,
        })
    }

    pub fn load_utf8<T: arrow::datatypes::ArrowPrimitiveType>(
        self,
        field: &str,
    ) -> eyre::Result<Self> {
        let mut array_data = self.array_data;
        let mut buffers = self.buffers;
        let mut offset_buffers = self.offset_buffers;

        let data = array_data.remove(field).ok_or_eyre(eyre::eyre!(format!(
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

    pub fn primitive_array<'a, T: arrow::datatypes::ArrowPrimitiveType>(
        &'a self,
        field: &str,
    ) -> eyre::Result<&'a [T::Native]> {
        let buffer = self.buffers.get(field).ok_or_eyre(eyre::eyre!(format!(
            "Invalid field {} for this map of data",
            field
        )))?;

        let slice = buffer.typed_data::<T::Native>();

        Ok(slice)
    }

    pub fn utf8_array(&mut self, _field: &str) -> eyre::Result<Vec<String>> {
        Err(eyre::eyre!("Not implemented"))
    }
}
