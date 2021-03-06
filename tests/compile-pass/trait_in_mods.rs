// Make sure that everything compiles even without the prelude. This basically
// forces us to generate full paths for types of the standard/core library.
#![no_implicit_prelude]


mod outer {
    use auto_impl::auto_impl;
    use std::{
        string::String,
        result::Result,
    };

    #[auto_impl(Fn)]
    trait Foo<'a, T> {
        fn execute<'b>(
            &'a self,
            arg1: &'b T,
            arg3: &'static str,
        ) -> Result<T, String>;
    }

    #[auto_impl(&, &mut, Box, Rc, Arc)]
    trait Bar<'a, T> {
        fn execute<'b>(
            &'a self,
            arg1: &'b T,
            arg3: &'static str,
        ) -> Result<T, String>;
    }

    mod inner {
        use auto_impl::auto_impl;
        use std::{
            string::String,
            result::Result,
        };

        #[auto_impl(Fn)]
        trait Foo<'a, T> {
            fn execute<'b>(
                &'a self,
                arg1: &'b T,
                arg3: &'static str,
            ) -> Result<T, String>;
        }

        #[auto_impl(&, &mut, Box, Rc, Arc)]
        trait Bar<'a, T> {
            fn execute<'b>(
                &'a self,
                arg1: &'b T,
                arg3: &'static str,
            ) -> Result<T, String>;
        }
    }
}
