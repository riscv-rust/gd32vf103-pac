#[doc = "Register `mtimecmp_lo` reader"]
pub type R = crate::R<MTIMECMP_LO_SPEC>;
#[doc = "Register `mtimecmp_lo` writer"]
pub type W = crate::W<MTIMECMP_LO_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MTIMECMP_LO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Timer comparison value (lower half)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIMECMP_LO_SPEC;
impl crate::RegisterSpec for MTIMECMP_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimecmp_lo::R`](R) reader structure"]
impl crate::Readable for MTIMECMP_LO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtimecmp_lo::W`](W) writer structure"]
impl crate::Writable for MTIMECMP_LO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mtimecmp_lo to value 0xffff_ffff"]
impl crate::Resettable for MTIMECMP_LO_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
