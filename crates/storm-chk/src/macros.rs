macro_rules! impl_fmt {
  ($ty:ident -> $func:ident as $trait:ident) => {
    impl ::core::fmt::$trait for $ty {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::$trait::fmt(&self.$func(), f)
      }
    }
  };
}

macro_rules! delegate_fmt {
  ($ty:ident -> $func:ident) => {
    impl_fmt!($ty -> $func as Debug);
    impl_fmt!($ty -> $func as Display);
  };
}
