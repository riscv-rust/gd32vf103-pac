#[doc = "Register `mstop` reader"]
pub type R = crate::R<MSTOP_SPEC>;
#[doc = "Register `mstop` writer"]
pub type W = crate::W<MSTOP_SPEC>;
#[doc = "Field `TIMESTOP` reader - Pause (1) or run (0) the timer"]
pub type TIMESTOP_R = crate::BitReader;
#[doc = "Field `TIMESTOP` writer - Pause (1) or run (0) the timer"]
pub type TIMESTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pause (1) or run (0) the timer"]
    #[inline(always)]
    pub fn timestop(&self) -> TIMESTOP_R {
        TIMESTOP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pause (1) or run (0) the timer"]
    #[inline(always)]
    #[must_use]
    pub fn timestop(&mut self) -> TIMESTOP_W<MSTOP_SPEC> {
        TIMESTOP_W::new(self, 0)
    }
}
#[doc = "Timer control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mstop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSTOP_SPEC;
impl crate::RegisterSpec for MSTOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstop::R`](R) reader structure"]
impl crate::Readable for MSTOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mstop::W`](W) writer structure"]
impl crate::Writable for MSTOP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mstop to value 0"]
impl crate::Resettable for MSTOP_SPEC {
    const RESET_VALUE: u32 = 0;
}
