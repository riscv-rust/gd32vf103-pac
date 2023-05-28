#[doc = "Register `RFIFOMP0` reader"]
pub struct R(crate::R<RFIFOMP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIFOMP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIFOMP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIFOMP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DLENC` reader - Data length code"]
pub type DLENC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FI` reader - Filtering index"]
pub type FI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS` reader - Time stamp"]
pub type TS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlenc(&self) -> DLENC_R {
        DLENC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Filtering index"]
    #[inline(always)]
    pub fn fi(&self) -> FI_R {
        FI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Time stamp"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Receive FIFO0 mailbox property register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfifomp0](index.html) module"]
pub struct RFIFOMP0_SPEC;
impl crate::RegisterSpec for RFIFOMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfifomp0::R](R) reader structure"]
impl crate::Readable for RFIFOMP0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RFIFOMP0 to value 0"]
impl crate::Resettable for RFIFOMP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
