#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::READYNEXT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `READYNEXT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READYNEXTR {
    #[doc = "NVMC cannot accept any write operation"]
    BUSY,
    #[doc = "NVMC is ready"]
    READY,
}
impl READYNEXTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            READYNEXTR::BUSY => false,
            READYNEXTR::READY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READYNEXTR {
        match value {
            false => READYNEXTR::BUSY,
            true => READYNEXTR::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline]
    pub fn is_busy(&self) -> bool {
        *self == READYNEXTR::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == READYNEXTR::READY
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - NVMC can accept a new write operation"]
    #[inline]
    pub fn readynext(&self) -> READYNEXTR {
        READYNEXTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
