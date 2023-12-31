macro_rules! impl_indv_shift_ops {
    ($t:ty, $tname:ident, $fname:ident, $w:ident, $ta_name:ident, $fa_name:ident) => {
        impl $tname<usize> for $t {
            type Output = $t;

            #[inline]
            fn $fname(self, rhs: usize) -> $t {
                Self::from((self.0).$w(rhs as u32))
            }
        }

        impl $ta_name<usize> for $t {
            #[inline]
            fn $fa_name(&mut self, rhs: usize) {
                *self = Self::from((self.0).$w(rhs as u32));
            }
        }
    };
}

macro_rules! impl_indv_bit_ops {
    ($t:ty, $b:ty, $tname:ident, $fname:ident, $w:ident, $ta_name:ident, $fa_name:ident) => {
        impl $tname for $t {
            type Output = $t;

            #[inline]
            fn $fname(self, rhs: $t) -> $t {
                Self::from((self.0).$w(rhs.0))
            }
        }

        impl $ta_name for $t {
            #[inline]
            fn $fa_name(&mut self, rhs: $t) {
                *self = Self::from((self.0).$w(rhs.0));
            }
        }

        impl $tname<$b> for $t {
            type Output = $t;

            #[inline]
            fn $fname(self, rhs: $b) -> $t {
                Self::from((self.0).$w(rhs))
            }
        }

        impl $ta_name<$b> for $t {
            #[inline]
            fn $fa_name(&mut self, rhs: $b) {
                *self = Self::from((self.0).$w(rhs));
            }
        }
    };
}

macro_rules! impl_bit_ops {
    ($t:tt, $b:tt) => {
        impl From<$b> for $t {
            fn from(bit_type: $b) -> Self {
                $t(bit_type)
            }
        }

        impl From<$t> for $b {
            fn from(it: $t) -> Self {
                it.0
            }
        }

        impl_indv_bit_ops!($t, $b, Rem, rem, rem, RemAssign, rem_assign);
        impl_indv_bit_ops!($t, $b, BitOr, bitor, bitor, BitOrAssign, bitor_assign);
        impl_indv_bit_ops!($t, $b, BitAnd, bitand, bitand, BitAndAssign, bitand_assign);
        impl_indv_bit_ops!($t, $b, BitXor, bitxor, bitxor, BitXorAssign, bitxor_assign);

        impl_indv_bit_ops!($t, $b, Add, add, wrapping_add, AddAssign, add_assign);
        impl_indv_bit_ops!($t, $b, Div, div, wrapping_div, DivAssign, div_assign);
        impl_indv_bit_ops!($t, $b, Mul, mul, wrapping_mul, MulAssign, mul_assign);
        impl_indv_bit_ops!($t, $b, Sub, sub, wrapping_sub, SubAssign, sub_assign);

        impl_indv_shift_ops!($t, Shl, shl, wrapping_shl, ShlAssign, shl_assign);
        impl_indv_shift_ops!($t, Shr, shr, wrapping_shr, ShrAssign, shr_assign);

        impl Not for $t {
            type Output = $t;

            #[inline]
            fn not(self) -> $t {
                $t(!self.0)
            }
        }
    };
}