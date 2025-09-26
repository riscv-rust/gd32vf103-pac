#[doc = "Register `CNT` reader"]
pub type R = crate::R<CNT_SPEC>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CNT_SPEC>;
#[doc = "Field `CNT` reader - current counter value"]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - current counter value"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - current counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - current counter value"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<CNT_SPEC> {
        CNT_W::new(self, 0)
    }
}
#[doc = "counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNT_SPEC {
    const RESET_VALUE: u16 = 0;
}
