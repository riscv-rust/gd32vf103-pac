#[doc = "Register `CNTH` reader"]
pub type R = crate::R<CNTH_SPEC>;
#[doc = "Register `CNTH` writer"]
pub type W = crate::W<CNTH_SPEC>;
#[doc = "Field `CNT` reader - RTC counter value high"]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - RTC counter value high"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC counter value high"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC counter value high"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<CNTH_SPEC> {
        CNT_W::new(self, 0)
    }
}
#[doc = "RTC counter high register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTH_SPEC;
impl crate::RegisterSpec for CNTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnth::R`](R) reader structure"]
impl crate::Readable for CNTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnth::W`](W) writer structure"]
impl crate::Writable for CNTH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTH to value 0"]
impl crate::Resettable for CNTH_SPEC {
    const RESET_VALUE: u32 = 0;
}
