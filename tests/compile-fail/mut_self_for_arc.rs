#![feature(use_extern_macros)]

extern crate auto_impl;

use auto_impl::auto_impl;


#[auto_impl(Arc)]
trait Foo {
    fn foo(&mut self);
}