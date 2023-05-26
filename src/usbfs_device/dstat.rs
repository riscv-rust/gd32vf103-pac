#[doc = "Register `DSTAT` reader"]
pub struct R(crate::R<DSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPST` reader - Suspend status"]
pub type SPST_R = crate::BitReader<bool>;
#[doc = "Field `ES` reader - Enumerated speed"]
pub type ES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FNRSOF` reader - Frame number of the received SOF"]
pub type FNRSOF_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Suspend status"]
    #[inline(always)]
    pub fn spst(&self) -> SPST_R {
        SPST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated speed"]
    #[inline(always)]
    pub fn es(&self) -> ES_R {
        ES_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 8:21 - Frame number of the received SOF"]
    #[inline(always)]
    pub fn fnrsof(&self) -> FNRSOF_R {
        FNRSOF_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
#[doc = "device status register (DSTAT)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dstat](index.html) module"]
pub struct DSTAT_SPEC;
impl crate::RegisterSpec for DSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dstat::R](R) reader structure"]
impl crate::Readable for DSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSTAT to value 0"]
impl crate::Resettable for DSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
