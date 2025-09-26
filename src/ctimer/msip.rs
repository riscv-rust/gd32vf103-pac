#[doc = "Register `msip` reader"]
pub type R = crate::R<MSIP_SPEC>;
#[doc = "Register `msip` writer"]
pub type W = crate::W<MSIP_SPEC>;
#[doc = "Field `MSIP` reader - Generate software interrupts"]
pub type MSIP_R = crate::BitReader;
#[doc = "Field `MSIP` writer - Generate software interrupts"]
pub type MSIP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Generate software interrupts"]
    #[inline(always)]
    pub fn msip(&self) -> MSIP_R {
        MSIP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Generate software interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn msip(&mut self) -> MSIP_W<MSIP_SPEC, 0> {
        MSIP_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSIP_SPEC;
impl crate::RegisterSpec for MSIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msip::R`](R) reader structure"]
impl crate::Readable for MSIP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msip::W`](W) writer structure"]
impl crate::Writable for MSIP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets msip to value 0"]
impl crate::Resettable for MSIP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
