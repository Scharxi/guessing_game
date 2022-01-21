macro_rules! enum_str {
        (enum $name:ident {
            $($variant:ident),*,
        }) => {
            pub enum $name {
                $($variant),*
            }

            impl $name {
                pub fn name(&self) -> &'static str {
                    match self {
                        $($name::$variant => stringify!($variant)),*
                    }
                }
            }
        };
    }

pub(crate) use enum_str;
