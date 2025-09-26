#[doc = "Register `CLICINFO` reader"]
pub type R = crate::R<CLICINFO_SPEC>;
#[doc = "Field `NUM_INTERRUPT` reader - NUM_INTERRUPT"]
pub type NUM_INTERRUPT_R = crate::FieldReader<u16>;
#[doc = "Field `VERSION` reader - VERSION"]
pub type VERSION_R = crate::FieldReader;
#[doc = "Field `CLICINTCTLBITS` reader - CLICINTCTLBITS"]
pub type CLICINTCTLBITS_R = crate::FieldReader;
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
#[doc = "clicinfo Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clicinfo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLICINFO_SPEC;
impl crate::RegisterSpec for CLICINFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clicinfo::R`](R) reader structure"]
impl crate::Readable for CLICINFO_SPEC {}
#[doc = "`reset()` method sets CLICINFO to value 0"]
impl crate::Resettable for CLICINFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
