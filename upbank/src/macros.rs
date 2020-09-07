#[macro_export]
macro_rules! setter {
    ($name:ident, $type:ty) => {
        pub fn $name(&mut self, value: $type) -> &mut Self {
            self.$name = Some(value);
            self
        }
    };
}


#[cfg(test)]
#[macro_export]
macro_rules! test_deserialization {
    ($name:ident, $file_name:expr, $type:ty) => {
        #[test]
        fn $name() {
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let mut path = std::path::PathBuf::from(manifest_dir);
            path.push("data");
            path.push($file_name);
            let contents = std::fs::read_to_string(path).unwrap();
            let _ = serde_json::from_str::<SuccessfulResponse<$type>>(&contents).unwrap();
        }
    };
}