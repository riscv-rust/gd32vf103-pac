#[doc = "Register `EXTISS2` reader"]
pub type R = crate::R<EXTISS2_SPEC>;
#[doc = "Register `EXTISS2` writer"]
pub type W = crate::W<EXTISS2_SPEC>;
#[doc = "Field `EXTI8_SS` reader - EXTI 8 sources selection"]
pub type EXTI8_SS_R = crate::FieldReader;
#[doc = "Field `EXTI8_SS` writer - EXTI 8 sources selection"]
pub type EXTI8_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTI9_SS` reader - EXTI 9 sources selection"]
pub type EXTI9_SS_R = crate::FieldReader;
#[doc = "Field `EXTI9_SS` writer - EXTI 9 sources selection"]
pub type EXTI9_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTI10_SS` reader - EXTI 10 sources selection"]
pub type EXTI10_SS_R = crate::FieldReader;
#[doc = "Field `EXTI10_SS` writer - EXTI 10 sources selection"]
pub type EXTI10_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTI11_SS` reader - EXTI 11 sources selection"]
pub type EXTI11_SS_R = crate::FieldReader;
#[doc = "Field `EXTI11_SS` writer - EXTI 11 sources selection"]
pub type EXTI11_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - EXTI 8 sources selection"]
    #[inline(always)]
    pub fn exti8_ss(&self) -> EXTI8_SS_R {
        EXTI8_SS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 9 sources selection"]
    #[inline(always)]
    pub fn exti9_ss(&self) -> EXTI9_SS_R {
        EXTI9_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 10 sources selection"]
    #[inline(always)]
    pub fn exti10_ss(&self) -> EXTI10_SS_R {
        EXTI10_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 11 sources selection"]
    #[inline(always)]
    pub fn exti11_ss(&self) -> EXTI11_SS_R {
        EXTI11_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 8 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti8_ss(&mut self) -> EXTI8_SS_W<EXTISS2_SPEC, 0> {
        EXTI8_SS_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 9 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti9_ss(&mut self) -> EXTI9_SS_W<EXTISS2_SPEC, 4> {
        EXTI9_SS_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 10 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti10_ss(&mut self) -> EXTI10_SS_W<EXTISS2_SPEC, 8> {
        EXTI10_SS_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI 11 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti11_ss(&mut self) -> EXTI11_SS_W<EXTISS2_SPEC, 12> {
        EXTI11_SS_W::new(self)
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
#[doc = "EXTI sources selection register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTISS2_SPEC;
impl crate::RegisterSpec for EXTISS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extiss2::R`](R) reader structure"]
impl crate::Readable for EXTISS2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extiss2::W`](W) writer structure"]
impl crate::Writable for EXTISS2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTISS2 to value 0"]
impl crate::Resettable for EXTISS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
