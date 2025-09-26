#[doc = "Register `CLICINTIE` reader"]
pub type R = crate::R<CLICINTIE_SPEC>;
#[doc = "Register `CLICINTIE` writer"]
pub type W = crate::W<CLICINTIE_SPEC>;
#[doc = "Field `IE` reader - IE"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - IE"]
pub type IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - IE"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IE"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<CLICINTIE_SPEC, 0> {
        IE_W::new(self)
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
#[doc = "clicintie Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clicintie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clicintie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLICINTIE_SPEC;
impl crate::RegisterSpec for CLICINTIE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clicintie::R`](R) reader structure"]
impl crate::Readable for CLICINTIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clicintie::W`](W) writer structure"]
impl crate::Writable for CLICINTIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLICINTIE to value 0"]
impl crate::Resettable for CLICINTIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
