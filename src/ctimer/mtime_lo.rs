#[doc = "Register `mtime_lo` reader"]
pub type R = crate::R<MTIME_LO_SPEC>;
#[doc = "Register `mtime_lo` writer"]
pub type W = crate::W<MTIME_LO_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer value (lower half)\n\nYou can [`read`](crate::Reg::read) this register and get [`mtime_lo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime_lo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIME_LO_SPEC;
impl crate::RegisterSpec for MTIME_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtime_lo::R`](R) reader structure"]
impl crate::Readable for MTIME_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtime_lo::W`](W) writer structure"]
impl crate::Writable for MTIME_LO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mtime_lo to value 0"]
impl crate::Resettable for MTIME_LO_SPEC {
    const RESET_VALUE: u32 = 0;
}
