#[doc = "Register `mtimecmp_lo` reader"]
pub type R = crate::R<MTIMECMP_LO_SPEC>;
#[doc = "Register `mtimecmp_lo` writer"]
pub type W = crate::W<MTIMECMP_LO_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer comparison value (lower half)\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp_lo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp_lo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIMECMP_LO_SPEC;
impl crate::RegisterSpec for MTIMECMP_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimecmp_lo::R`](R) reader structure"]
impl crate::Readable for MTIMECMP_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtimecmp_lo::W`](W) writer structure"]
impl crate::Writable for MTIMECMP_LO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mtimecmp_lo to value 0xffff_ffff"]
impl crate::Resettable for MTIMECMP_LO_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
