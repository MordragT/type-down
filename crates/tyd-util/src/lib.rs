pub trait TryAsRef<T> {
    fn try_as_ref(&self) -> Option<&T>;
}

pub trait TryAsMut<T> {
    fn try_as_mut(&mut self) -> Option<&mut T>;
}

#[macro_export]
macro_rules! impl_try_as {
    ($enum_type:ident, $($variant:ident($variant_type:ty)),*) => {
        $(
            impl TryAsRef<$variant_type> for $enum_type {
                fn try_as_ref(&self) -> Option<&$variant_type> {
                    match self {
                        $enum_type::$variant(val) => Some(val),
                        _ => None,
                    }
                }
            }

            impl TryAsMut<$variant_type> for $enum_type {
                fn try_as_mut(&mut self) -> Option<&mut $variant_type> {
                    match self {
                        $enum_type::$variant(val) => Some(val),
                        _ => None,
                    }
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_try_as_ref {
    ($enum_type:ident, $($variant:ident($variant_type:ty)),*) => {
        $(
            impl TryAsRef<$variant_type> for $enum_type {
                fn try_as_ref(&self) -> Option<&$variant_type> {
                    match self {
                        $enum_type::$variant(val) => Some(val),
                        _ => None,
                    }
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_try_as_mut {
    ($enum_type:ident, $($variant:ident($variant_type:ty)),*) => {
        $(
            impl TryAsMut<$variant_type> for $enum_type {
                fn try_as_mut(&mut self) -> Option<&mut $variant_type> {
                    match self {
                        $enum_type::$variant(val) => Some(val),
                        _ => None,
                    }
                }
            }
        )*
    };
}
