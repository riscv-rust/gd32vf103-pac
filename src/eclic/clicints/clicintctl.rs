#[doc = "Register `CLICINTCTL` reader"]
pub type R = crate::R<CLICINTCTL_SPEC>;
#[doc = "Register `CLICINTCTL` writer"]
pub type W = crate::W<CLICINTCTL_SPEC>;
#[doc = "Field `LEVEL_PRIORITY` reader - LEVEL_PRIORITY"]
pub type LEVEL_PRIORITY_R = crate::FieldReader;
#[doc = "Field `LEVEL_PRIORITY` writer - LEVEL_PRIORITY"]
pub type LEVEL_PRIORITY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
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
    pub fn level_priority(&mut self) -> LEVEL_PRIORITY_W<CLICINTCTL_SPEC, 0> {
        LEVEL_PRIORITY_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "clicintctl Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clicintctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clicintctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLICINTCTL_SPEC;
impl crate::RegisterSpec for CLICINTCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clicintctl::R`](R) reader structure"]
impl crate::Readable for CLICINTCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clicintctl::W`](W) writer structure"]
impl crate::Writable for CLICINTCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLICINTCTL to value 0"]
impl crate::Resettable for CLICINTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
