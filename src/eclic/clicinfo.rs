#[doc = "Register `CLICINFO` reader"]
pub struct R(crate::R<CLICINFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLICINFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLICINFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLICINFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUM_INTERRUPT` reader - NUM_INTERRUPT"]
pub type NUM_INTERRUPT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VERSION` reader - VERSION"]
pub type VERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLICINTCTLBITS` reader - CLICINTCTLBITS"]
pub type CLICINTCTLBITS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:12 - NUM_INTERRUPT"]
    #[inline(always)]
    pub fn num_interrupt(&self) -> NUM_INTERRUPT_R {
        NUM_INTERRUPT_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:20 - VERSION"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:24 - CLICINTCTLBITS"]
    #[inline(always)]
    pub fn clicintctlbits(&self) -> CLICINTCTLBITS_R {
        CLICINTCTLBITS_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
#[doc = "clicinfo Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clicinfo](index.html) module"]
pub struct CLICINFO_SPEC;
impl crate::RegisterSpec for CLICINFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clicinfo::R](R) reader structure"]
impl crate::Readable for CLICINFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLICINFO to value 0"]
impl crate::Resettable for CLICINFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
