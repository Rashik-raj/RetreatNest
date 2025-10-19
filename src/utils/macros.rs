#[macro_export]
macro_rules! set_fields {
    ($model:expr, $payload:expr, $( $field:ident ),* ) => {
        $(
            if let Some(value) = $payload.$field.clone() {
                $model.$field = Set(value);
            }
        )*
    };
}

#[macro_export]
macro_rules! set_active_model_fields {
    ($payload:expr, $model:ident, { $( $field:ident ),* $(,)? }) => {
        $model {
            $( $field: Set($payload.$field.clone()), )*
            ..Default::default()
        }
    };
}

#[macro_export]
macro_rules! map_fields {
    ($from:expr, $to:ident, { $( $field:ident ),* $(,)? }) => {
        $to {
            $( $field: $from.$field, )*
        }
    };
}


