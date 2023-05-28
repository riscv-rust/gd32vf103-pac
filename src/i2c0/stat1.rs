#[doc = "Register `STAT1` reader"]
pub struct R(crate::R<STAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MASTER` reader - A flag indicating whether I2C block is in master or slave mode"]
pub type MASTER_R = crate::BitReader<bool>;
#[doc = "Field `I2CBSY` reader - Busy flag"]
pub type I2CBSY_R = crate::BitReader<bool>;
#[doc = "Field `TR` reader - Whether the I2C is a transmitter or a receiver"]
pub type TR_R = crate::BitReader<bool>;
#[doc = "Field `RXGC` reader - General call address (00h) received"]
pub type RXGC_R = crate::BitReader<bool>;
#[doc = "Field `DEFSMB` reader - Default address of SMBusDevice"]
pub type DEFSMB_R = crate::BitReader<bool>;
#[doc = "Field `HSTSMB` reader - SMBus Host Header detected in slave mode"]
pub type HSTSMB_R = crate::BitReader<bool>;
#[doc = "Field `DUMODF` reader - Dual Flag in slave mode"]
pub type DUMODF_R = crate::BitReader<bool>;
#[doc = "Field `PECV` reader - Packet Error Checking Value that calculated by hardware when PEC is enabled"]
pub type PECV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - A flag indicating whether I2C block is in master or slave mode"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Busy flag"]
    #[inline(always)]
    pub fn i2cbsy(&self) -> I2CBSY_R {
        I2CBSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Whether the I2C is a transmitter or a receiver"]
    #[inline(always)]
    pub fn tr(&self) -> TR_R {
        TR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - General call address (00h) received"]
    #[inline(always)]
    pub fn rxgc(&self) -> RXGC_R {
        RXGC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Default address of SMBusDevice"]
    #[inline(always)]
    pub fn defsmb(&self) -> DEFSMB_R {
        DEFSMB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus Host Header detected in slave mode"]
    #[inline(always)]
    pub fn hstsmb(&self) -> HSTSMB_R {
        HSTSMB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Dual Flag in slave mode"]
    #[inline(always)]
    pub fn dumodf(&self) -> DUMODF_R {
        DUMODF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Packet Error Checking Value that calculated by hardware when PEC is enabled"]
    #[inline(always)]
    pub fn pecv(&self) -> PECV_R {
        PECV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Transfer status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat1](index.html) module"]
pub struct STAT1_SPEC;
impl crate::RegisterSpec for STAT1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [stat1::R](R) reader structure"]
impl crate::Readable for STAT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT1 to value 0"]
impl crate::Resettable for STAT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
