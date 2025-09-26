#[doc = "Register `CREP` reader"]
pub type R = crate::R<CREP_SPEC>;
#[doc = "Register `CREP` writer"]
pub type W = crate::W<CREP_SPEC>;
#[doc = "Field `CREP` reader - Counter repetition value"]
pub type CREP_R = crate::FieldReader;
#[doc = "Field `CREP` writer - Counter repetition value"]
pub type CREP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Counter repetition value"]
    #[inline(always)]
    pub fn crep(&self) -> CREP_R {
        CREP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Counter repetition value"]
    #[inline(always)]
    #[must_use]
    pub fn crep(&mut self) -> CREP_W<CREP_SPEC> {
        CREP_W::new(self, 0)
    }
}
#[doc = "Counter repetition register\n\nYou can [`read`](crate::Reg::read) this register and get [`crep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CREP_SPEC;
impl crate::RegisterSpec for CREP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`crep::R`](R) reader structure"]
impl crate::Readable for CREP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crep::W`](W) writer structure"]
impl crate::Writable for CREP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CREP to value 0"]
impl crate::Resettable for CREP_SPEC {
    const RESET_VALUE: u16 = 0;
}
