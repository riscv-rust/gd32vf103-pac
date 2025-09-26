#[doc = "Register `CLICINTCTL` reader"]
pub type R = crate::R<CLICINTCTL_SPEC>;
#[doc = "Register `CLICINTCTL` writer"]
pub type W = crate::W<CLICINTCTL_SPEC>;
#[doc = "Field `LEVEL_PRIORITY` reader - LEVEL_PRIORITY"]
pub type LEVEL_PRIORITY_R = crate::FieldReader;
#[doc = "Field `LEVEL_PRIORITY` writer - LEVEL_PRIORITY"]
pub type LEVEL_PRIORITY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - LEVEL_PRIORITY"]
    #[inline(always)]
    pub fn level_priority(&self) -> LEVEL_PRIORITY_R {
        LEVEL_PRIORITY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - LEVEL_PRIORITY"]
    #[inline(always)]
    #[must_use]
    pub fn level_priority(&mut self) -> LEVEL_PRIORITY_W<CLICINTCTL_SPEC> {
        LEVEL_PRIORITY_W::new(self, 0)
    }
}
#[doc = "clicintctl Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clicintctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clicintctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLICINTCTL_SPEC;
impl crate::RegisterSpec for CLICINTCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clicintctl::R`](R) reader structure"]
impl crate::Readable for CLICINTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clicintctl::W`](W) writer structure"]
impl crate::Writable for CLICINTCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CLICINTCTL to value 0"]
impl crate::Resettable for CLICINTCTL_SPEC {
    const RESET_VALUE: u8 = 0;
}
