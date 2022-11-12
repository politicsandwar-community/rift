#[macro_export]
macro_rules! convert {
    ($ctx:ident, $arg:ident = $type:ident) => {
        use $crate::traits::Convert;
        let $arg = $type::convert(&$ctx, $arg).await?;
    };
}

#[macro_export]
macro_rules! convert_option {
    ($ctx:ident, $arg:ident = $type:ident) => {
        use $crate::traits::Convert;
        let $arg = $type::convert_option(&$ctx, $arg).await?;
    };
}
