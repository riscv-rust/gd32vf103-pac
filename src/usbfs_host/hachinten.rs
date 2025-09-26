#[doc = "Register `HACHINTEN` reader"]
pub type R = crate::R<HACHINTEN_SPEC>;
#[doc = "Register `HACHINTEN` writer"]
pub type W = crate::W<HACHINTEN_SPEC>;
#[doc = "Field `CINTEN` reader - Channel interrupt enable"]
pub type CINTEN_R = crate::FieldReader;
#[doc = "Field `CINTEN` writer - Channel interrupt enable"]
pub type CINTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Channel interrupt enable"]
    #[inline(always)]
    pub fn cinten(&self) -> CINTEN_R {
        CINTEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Channel interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cinten(&mut self) -> CINTEN_W<HACHINTEN_SPEC> {
        CINTEN_W::new(self, 0)
    }
}
#[doc = "host all channels interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`hachinten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hachinten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HACHINTEN_SPEC;
impl crate::RegisterSpec for HACHINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hachinten::R`](R) reader structure"]
impl crate::Readable for HACHINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hachinten::W`](W) writer structure"]
impl crate::Writable for HACHINTEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HACHINTEN to value 0"]
impl crate::Resettable for HACHINTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
