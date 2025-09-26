#[doc = "Register `CH4CNT` reader"]
pub type R = crate::R<CH4CNT_SPEC>;
#[doc = "Register `CH4CNT` writer"]
pub type W = crate::W<CH4CNT_SPEC>;
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
    pub fn cnt(&mut self) -> CNT_W<CH4CNT_SPEC> {
        CNT_W::new(self, 0)
    }
}
#[doc = "Channel 4 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4CNT_SPEC;
impl crate::RegisterSpec for CH4CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4cnt::R`](R) reader structure"]
impl crate::Readable for CH4CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch4cnt::W`](W) writer structure"]
impl crate::Writable for CH4CNT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH4CNT to value 0"]
impl crate::Resettable for CH4CNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
