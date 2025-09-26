#[doc = "Register `RFIFOMP0` reader"]
pub type R = crate::R<RFIFOMP0_SPEC>;
#[doc = "Field `DLENC` reader - Data length code"]
pub type DLENC_R = crate::FieldReader;
#[doc = "Field `FI` reader - Filtering index"]
pub type FI_R = crate::FieldReader;
#[doc = "Field `TS` reader - Time stamp"]
pub type TS_R = crate::FieldReader<u16>;
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
#[doc = "Receive FIFO0 mailbox property register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfifomp0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFIFOMP0_SPEC;
impl crate::RegisterSpec for RFIFOMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifomp0::R`](R) reader structure"]
impl crate::Readable for RFIFOMP0_SPEC {}
#[doc = "`reset()` method sets RFIFOMP0 to value 0"]
impl crate::Resettable for RFIFOMP0_SPEC {
    const RESET_VALUE: u32 = 0;
}
