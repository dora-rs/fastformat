use arrow::array::{StringArray, UInt32Array, UnionArray};
use arrow::buffer::ScalarBuffer;
use arrow::datatypes::{DataType, Field, UnionFields};
use arrow::error::ArrowError;
use std::path::PathBuf;
use std::sync::Arc;

pub struct ImageSequence {
    path: PathBuf,
    framerate: u32,
    name: String,
}

impl ImageSequence {
    pub fn from_flat(path: PathBuf, framerate: u32, name: String) -> Self {
        ImageSequence {
            path,
            framerate,
            name,
        }
    }

    pub fn from_arrow_array(arrow_array: UnionArray) -> Self {
        let path = arrow_array
            .child(0)
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .value(0);
        let framerate = arrow_array
            .child(1)
            .unwrap()
            .as_any()
            .downcast_ref::<UInt32Array>()
            .value(0);
        let name = arrow_array
            .child(2)
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .value(0);

        ImageSequence {
            path,
            framerate,
            name,
        }
    }

    pub fn to_arrow_array(self) -> Result<UnionArray, ArrowError> {
        let path = StringArray::from(vec![self.path.to_str().unwrap()]);
        let framerate = UInt32Array::from(vec![self.framerate]);
        let name = StringArray::from(vec![self.name]);

        let type_ids = [].into_iter().collect::<ScalarBuffer<i8>>();
        let offsets = [].into_iter().collect::<ScalarBuffer<i32>>();

        let union_fields = [
            (0, Arc::new(Field::new("Path", DataType::Utf8, false))),
            (
                1,
                Arc::new(Field::new("Framerate", DataType::UInt32, false)),
            ),
            (2, Arc::new(Field::new("Name", DataType::Utf8, false))),
        ]
        .into_iter()
        .collect::<UnionFields>();

        let children: Vec<Arc<dyn arrow::array::Array>> =
            vec![Arc::new(path), Arc::new(framerate), Arc::new(name)];

        UnionArray::try_new(union_fields, type_ids, Some(offsets), children)
    }
}
