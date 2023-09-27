#[macro_export]
macro_rules! const_assert {
  ($($tt:tt)*) => {
    const _: () = assert!($($tt)*);
  };
}

#[macro_export]
macro_rules! const_assert_size {
  ($ty:ty, $size:expr) => {
    $crate::const_assert!(
      <$ty>::SIZE == $size,
      $crate::invalid_size!(stringify!($ty), stringify!($size)),
    );
  };
}

#[macro_export]
#[doc(hidden)]
macro_rules! invalid_size {
  ($ty:expr, $size:expr) => {
    concat!("Invalid ", $ty, " Size: Expected ", $size, " Bytes.")
  };
}
