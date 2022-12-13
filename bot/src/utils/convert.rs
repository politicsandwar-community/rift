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

#[macro_export]
macro_rules! convert_from_id {
    ($ctx:ident, $val:ident, $err:ident, $getter:ident) => {
        let res = $val.parse::<i32>();
        if let Ok(res) = res {
            let got = $ctx.data().cache.$getter(&res);
            if let Some(got) = got {
                return Ok(got);
            } else {
                return Err($crate::errors::NotFoundError::$err(Some($val)).into());
            }
        } else {
            return Err($crate::errors::NotFoundError::$err(Some($val)).into());
        }
    };
}

#[macro_export]
macro_rules! convert_from_option_id {
    ($ctx:ident, $val:ident, $err:ident, $getter:ident) => {
        if let Some(val) = $val {
            $crate::convert_from_id!($ctx, val, $err, $getter);
        } else {
            return Err($crate::errors::NotFoundError::$err(None).into());
        }
    };
}

#[macro_export]
macro_rules! impl_convert_from_option_id {
    ($model:ident, $getter:ident) => {
        #[async_trait::async_trait]
        impl $crate::traits::Convert for $model {
            async fn convert_option(
                ctx: &$crate::types::Context<'_>,
                val: Option<String>,
            ) -> Result<Self, $crate::types::Error> {
                $crate::convert_from_option_id!(ctx, val, $model, $getter);
            }
        }
    };
}
