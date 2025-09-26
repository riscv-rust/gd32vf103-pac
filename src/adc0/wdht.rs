#[doc = "Register `WDHT` reader"]
pub type R = crate::R<WDHT_SPEC>;
#[doc = "Register `WDHT` writer"]
pub type W = crate::W<WDHT_SPEC>;
#[doc = "Field `WDHT` reader - Analog watchdog higher threshold"]
pub type WDHT_R = crate::FieldReader<u16>;
#[doc = "Field `WDHT` writer - Analog watchdog higher threshold"]
pub type WDHT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog watchdog higher threshold"]
    #[inline(always)]
    pub fn wdht(&self) -> WDHT_R {
        WDHT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog higher threshold"]
    #[inline(always)]
    #[must_use]
    pub fn wdht(&mut self) -> WDHT_W<WDHT_SPEC> {
        WDHT_W::new(self, 0)
    }
}
#[doc = "watchdog higher threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdht::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdht::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDHT_SPEC;
impl crate::RegisterSpec for WDHT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdht::R`](R) reader structure"]
impl crate::Readable for WDHT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdht::W`](W) writer structure"]
impl crate::Writable for WDHT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDHT to value 0x0fff"]
impl crate::Resettable for WDHT_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
