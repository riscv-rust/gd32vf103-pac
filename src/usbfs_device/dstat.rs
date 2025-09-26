#[doc = "Register `DSTAT` reader"]
pub type R = crate::R<DSTAT_SPEC>;
#[doc = "Field `SPST` reader - Suspend status"]
pub type SPST_R = crate::BitReader;
#[doc = "Field `ES` reader - Enumerated speed"]
pub type ES_R = crate::FieldReader;
#[doc = "Field `FNRSOF` reader - Frame number of the received SOF"]
pub type FNRSOF_R = crate::FieldReader<u16>;
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
#[doc = "device status register (DSTAT)\n\nYou can [`read`](crate::Reg::read) this register and get [`dstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTAT_SPEC;
impl crate::RegisterSpec for DSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dstat::R`](R) reader structure"]
impl crate::Readable for DSTAT_SPEC {}
#[doc = "`reset()` method sets DSTAT to value 0"]
impl crate::Resettable for DSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
