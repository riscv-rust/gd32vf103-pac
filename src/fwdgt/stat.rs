#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Field `PUD` reader - Free watchdog timer prescaler value update"]
pub type PUD_R = crate::BitReader;
#[doc = "Field `RUD` reader - Free watchdog timer counter reload value update"]
pub type RUD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Free watchdog timer prescaler value update"]
    #[inline(always)]
    pub fn pud(&self) -> PUD_R {
        PUD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Free watchdog timer counter reload value update"]
    #[inline(always)]
    pub fn rud(&self) -> RUD_R {
        RUD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
