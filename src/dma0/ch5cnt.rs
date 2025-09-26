#[doc = "Register `CH5CNT` reader"]
pub type R = crate::R<CH5CNT_SPEC>;
#[doc = "Register `CH5CNT` writer"]
pub type W = crate::W<CH5CNT_SPEC>;
#[doc = "Field `CNT` reader - Transfer counter"]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Transfer counter"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transfer counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transfer counter"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<CH5CNT_SPEC> {
        CNT_W::new(self, 0)
    }
}
#[doc = "Channel 5 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH5CNT_SPEC;
impl crate::RegisterSpec for CH5CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5cnt::R`](R) reader structure"]
impl crate::Readable for CH5CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch5cnt::W`](W) writer structure"]
impl crate::Writable for CH5CNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH5CNT to value 0"]
impl crate::Resettable for CH5CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
