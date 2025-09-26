#[doc = "Register `EXTISS0` reader"]
pub type R = crate::R<EXTISS0_SPEC>;
#[doc = "Register `EXTISS0` writer"]
pub type W = crate::W<EXTISS0_SPEC>;
#[doc = "Field `EXTI0_SS` reader - EXTI 0 sources selection"]
pub type EXTI0_SS_R = crate::FieldReader;
#[doc = "Field `EXTI0_SS` writer - EXTI 0 sources selection"]
pub type EXTI0_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTI1_SS` reader - EXTI 1 sources selection"]
pub type EXTI1_SS_R = crate::FieldReader;
#[doc = "Field `EXTI1_SS` writer - EXTI 1 sources selection"]
pub type EXTI1_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTI2_SS` reader - EXTI 2 sources selection"]
pub type EXTI2_SS_R = crate::FieldReader;
#[doc = "Field `EXTI2_SS` writer - EXTI 2 sources selection"]
pub type EXTI2_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `EXTI3_SS` reader - EXTI 3 sources selection"]
pub type EXTI3_SS_R = crate::FieldReader;
#[doc = "Field `EXTI3_SS` writer - EXTI 3 sources selection"]
pub type EXTI3_SS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - EXTI 0 sources selection"]
    #[inline(always)]
    pub fn exti0_ss(&self) -> EXTI0_SS_R {
        EXTI0_SS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 1 sources selection"]
    #[inline(always)]
    pub fn exti1_ss(&self) -> EXTI1_SS_R {
        EXTI1_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 2 sources selection"]
    #[inline(always)]
    pub fn exti2_ss(&self) -> EXTI2_SS_R {
        EXTI2_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 3 sources selection"]
    #[inline(always)]
    pub fn exti3_ss(&self) -> EXTI3_SS_R {
        EXTI3_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 0 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti0_ss(&mut self) -> EXTI0_SS_W<EXTISS0_SPEC, 0> {
        EXTI0_SS_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 1 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti1_ss(&mut self) -> EXTI1_SS_W<EXTISS0_SPEC, 4> {
        EXTI1_SS_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 2 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti2_ss(&mut self) -> EXTI2_SS_W<EXTISS0_SPEC, 8> {
        EXTI2_SS_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI 3 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti3_ss(&mut self) -> EXTI3_SS_W<EXTISS0_SPEC, 12> {
        EXTI3_SS_W::new(self)
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
#[doc = "EXTI sources selection register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extiss0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extiss0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTISS0_SPEC;
impl crate::RegisterSpec for EXTISS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extiss0::R`](R) reader structure"]
impl crate::Readable for EXTISS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extiss0::W`](W) writer structure"]
impl crate::Writable for EXTISS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTISS0 to value 0"]
impl crate::Resettable for EXTISS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
