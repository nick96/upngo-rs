#[macro_export]
macro_rules! setter {
    ($name:ident, $type:ty) => {
        pub fn $name(&mut self, value: $type) -> &mut Self {
            self.$name = Some(value);
            self
        }
    };
}