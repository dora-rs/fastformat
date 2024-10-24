use std::sync::Arc;

#[derive(Default)]
pub struct ArrowDataBuilder {
    union_children: Vec<arrow::array::ArrayRef>,
    union_fields: Vec<(i8, arrow::datatypes::FieldRef)>,
}

impl ArrowDataBuilder {
    pub fn push_primitive_singleton<T: arrow::datatypes::ArrowPrimitiveType>(
        self,
        field: &str,
        value: T::Native,
    ) -> Self {
        let mut union_children = self.union_children;
        let mut union_fields = self.union_fields;

        let index = union_children.len();

        let data = Arc::new(arrow::array::PrimitiveArray::<T>::from_value(value, 1));
        union_children.push(data);

        let field = (
            index as i8,
            Arc::new(arrow::datatypes::Field::new(field, T::DATA_TYPE, false)),
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
    ) -> Self {
        let mut union_children = self.union_children;
        let mut union_fields = self.union_fields;

        let index = union_children.len();

        let data = Arc::new(arrow::array::PrimitiveArray::<T>::from_iter_values(value));
        union_children.push(data);

        let field = (
            index as i8,
            Arc::new(arrow::datatypes::Field::new(field, T::DATA_TYPE, false)),
        );
        union_fields.push(field);

        Self {
            union_children,
            union_fields,
        }
    }

    pub fn push_utf8_singleton(self, field: &str, value: String) -> Self {
        let mut union_children = self.union_children;
        let mut union_fields = self.union_fields;

        let index = union_children.len();

        let data = Arc::new(arrow::array::StringArray::from(vec![value]));
        union_children.push(data);

        let field = (
            index as i8,
            Arc::new(arrow::datatypes::Field::new(
                field,
                arrow::datatypes::DataType::Utf8,
                false,
            )),
        );
        union_fields.push(field);

        Self {
            union_children,
            union_fields,
        }
    }

    pub fn push_utf8_array(self, field: &str, value: Vec<String>) -> Self {
        let mut union_children = self.union_children;
        let mut union_fields = self.union_fields;

        let index = union_children.len();

        let data = Arc::new(arrow::array::StringArray::from(value));
        union_children.push(data);

        let field = (
            index as i8,
            Arc::new(arrow::datatypes::Field::new(
                field,
                arrow::datatypes::DataType::Utf8,
                false,
            )),
        );
        union_fields.push(field);

        Self {
            union_children,
            union_fields,
        }
    }

    pub fn build(self) -> eyre::Result<arrow::array::ArrayData> {
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
        .map_err(|e| eyre::eyre!(format!("Failed to create UnionArray: {}", e)))?
        .into_data())
    }
}
