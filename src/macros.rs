#[macro_export]
macro_rules! impl_component {
    ($struct_name:ident, $num:literal) => {
        impl Component for $struct_name {
            fn index() -> usize {
                $num
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }               
    };
}