#[doc = "Register `TPCTL` reader"]
pub type R = crate::R<TPCTL_SPEC>;
#[doc = "Register `TPCTL` writer"]
pub type W = crate::W<TPCTL_SPEC>;
#[doc = "Field `TPEN` reader - TAMPER detection enable"]
pub type TPEN_R = crate::BitReader;
#[doc = "Field `TPEN` writer - TAMPER detection enable"]
pub type TPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TPAL` reader - TAMPER pin active level"]
pub type TPAL_R = crate::BitReader;
#[doc = "Field `TPAL` writer - TAMPER pin active level"]
pub type TPAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - TAMPER detection enable"]
    #[inline(always)]
    pub fn tpen(&self) -> TPEN_R {
        TPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMPER pin active level"]
    #[inline(always)]
    pub fn tpal(&self) -> TPAL_R {
        TPAL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMPER detection enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpen(&mut self) -> TPEN_W<TPCTL_SPEC, 0> {
        TPEN_W::new(self)
    }
    #[doc = "Bit 1 - TAMPER pin active level"]
    #[inline(always)]
    #[must_use]
    pub fn tpal(&mut self) -> TPAL_W<TPCTL_SPEC, 1> {
        TPAL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Tamper pin control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TPCTL_SPEC;
impl crate::RegisterSpec for TPCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tpctl::R`](R) reader structure"]
impl crate::Readable for TPCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tpctl::W`](W) writer structure"]
impl crate::Writable for TPCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPCTL to value 0"]
impl crate::Resettable for TPCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
