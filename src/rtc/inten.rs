#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `SCIE` reader - Second interrupt"]
pub type SCIE_R = crate::BitReader;
#[doc = "Field `SCIE` writer - Second interrupt"]
pub type SCIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ALRMIE` reader - Alarm interrupt enable"]
pub type ALRMIE_R = crate::BitReader;
#[doc = "Field `ALRMIE` writer - Alarm interrupt enable"]
pub type ALRMIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OVIE` reader - Overflow interrupt enable"]
pub type OVIE_R = crate::BitReader;
#[doc = "Field `OVIE` writer - Overflow interrupt enable"]
pub type OVIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Second interrupt"]
    #[inline(always)]
    pub fn scie(&self) -> SCIE_R {
        SCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt enable"]
    #[inline(always)]
    pub fn alrmie(&self) -> ALRMIE_R {
        ALRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovie(&self) -> OVIE_R {
        OVIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Second interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn scie(&mut self) -> SCIE_W<INTEN_SPEC, 0> {
        SCIE_W::new(self)
    }
    #[doc = "Bit 1 - Alarm interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrmie(&mut self) -> ALRMIE_W<INTEN_SPEC, 1> {
        ALRMIE_W::new(self)
    }
    #[doc = "Bit 2 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovie(&mut self) -> OVIE_W<INTEN_SPEC, 2> {
        OVIE_W::new(self)
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
#[doc = "RTC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
