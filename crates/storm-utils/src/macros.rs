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

#[macro_export]
macro_rules! bitflags {
  (
    $(#[$meta:meta])*
    $vis:vis struct $name:ident: $ty:ty {
      $(
        $(#[$inner:ident $($args:tt)*])*
        const $flag:ident = $value:expr;
      )*
    }
  ) => {
    $crate::bitflags::bitflags! {
      $(#[$meta])*
      $vis struct $name: $ty {
        $(
          $(#[$inner $($args)*])*
          const $flag = $value;
        )*
      }
    }

    impl $name {
      #[cfg(debug_assertions)]
      #[inline]
      fn from_value(bits: $ty) -> Self {
        Self::from_bits(bits).expect(concat!("Invalid bits for `", stringify!($name), "`"))
      }

      #[cfg(not(debug_assertions))]
      #[inline]
      fn from_value(bits: $ty) -> Self {
        Self::from_bits_retain(bits)
      }
    }

    impl ::core::fmt::Display for $name {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        if !self.is_empty() {
          $crate::bitflags::parser::to_writer(self, f)
        } else {
          f.write_str("<<EMPTY>>")
        }
      }
    }
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! feature {
  (
    #[cfg(feature = $feature:literal)]
    $($item:item)*
  ) => {
    $(
      #[cfg(feature = $feature)]
      #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
      $item
    )*
  };
}

#[doc(hidden)]
#[macro_export]
macro_rules! only_serde {
  ($($item:item)*) => {
    $crate::feature! {
      #[cfg(feature = "serde")]
      $($item)*
    }
  };
}
