#[doc = "Register `CLICINTIP` reader"]
pub type R = crate::R<CLICINTIP_SPEC>;
#[doc = "Register `CLICINTIP` writer"]
pub type W = crate::W<CLICINTIP_SPEC>;
#[doc = "Field `IP` reader - IP"]
pub type IP_R = crate::BitReader;
#[doc = "Field `IP` writer - IP"]
pub type IP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - IP"]
    #[inline(always)]
    pub fn ip(&self) -> IP_R {
        IP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP"]
    #[inline(always)]
    #[must_use]
    pub fn ip(&mut self) -> IP_W<CLICINTIP_SPEC, 0> {
        IP_W::new(self)
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
#[doc = "clicintip Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clicintip::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clicintip::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLICINTIP_SPEC;
impl crate::RegisterSpec for CLICINTIP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clicintip::R`](R) reader structure"]
impl crate::Readable for CLICINTIP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clicintip::W`](W) writer structure"]
impl crate::Writable for CLICINTIP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLICINTIP to value 0"]
impl crate::Resettable for CLICINTIP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
