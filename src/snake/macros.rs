#[macro_export]
macro_rules! named {
    ($type:ty) => {
        |x, y| Box::new(<$type>::new(x, y))
    };
}

#[macro_export]
macro_rules! impl_entity {
    ($type:ident) => {
        impl $type {
            pub fn new(x: i16, y: i16) -> Self {
                Self { x, y }
            }
        }

        impl Entity for $type {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn x(&self) -> i16 {
                self.x
            }

            fn y(&self) -> i16 {
                self.y
            }
        }
    };
}
