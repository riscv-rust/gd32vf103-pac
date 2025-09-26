#[doc = "Register `CLICCFG` reader"]
pub type R = crate::R<CLICCFG_SPEC>;
#[doc = "Register `CLICCFG` writer"]
pub type W = crate::W<CLICCFG_SPEC>;
#[doc = "Field `NLBITS` reader - NLBITS"]
pub type NLBITS_R = crate::FieldReader;
#[doc = "Field `NLBITS` writer - NLBITS"]
pub type NLBITS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 1:4 - NLBITS"]
    #[inline(always)]
    pub fn nlbits(&self) -> NLBITS_R {
        NLBITS_R::new((self.bits >> 1) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 1:4 - NLBITS"]
    #[inline(always)]
    #[must_use]
    pub fn nlbits(&mut self) -> NLBITS_W<CLICCFG_SPEC, 1> {
        NLBITS_W::new(self)
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
#[doc = "cliccfg Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cliccfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cliccfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLICCFG_SPEC;
impl crate::RegisterSpec for CLICCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cliccfg::R`](R) reader structure"]
impl crate::Readable for CLICCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cliccfg::W`](W) writer structure"]
impl crate::Writable for CLICCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLICCFG to value 0"]
impl crate::Resettable for CLICCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
