#[doc = "Register `EXTISS1` reader"]
pub type R = crate::R<EXTISS1_SPEC>;
#[doc = "Register `EXTISS1` writer"]
pub type W = crate::W<EXTISS1_SPEC>;
#[doc = "Field `EXTI4_SS` reader - EXTI 4 sources selection"]
pub type EXTI4_SS_R = crate::FieldReader;
#[doc = "Field `EXTI4_SS` writer - EXTI 4 sources selection"]
pub type EXTI4_SS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI5_SS` reader - EXTI 5 sources selection"]
pub type EXTI5_SS_R = crate::FieldReader;
#[doc = "Field `EXTI5_SS` writer - EXTI 5 sources selection"]
pub type EXTI5_SS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI6_SS` reader - EXTI 6 sources selection"]
pub type EXTI6_SS_R = crate::FieldReader;
#[doc = "Field `EXTI6_SS` writer - EXTI 6 sources selection"]
pub type EXTI6_SS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI7_SS` reader - EXTI 7 sources selection"]
pub type EXTI7_SS_R = crate::FieldReader;
#[doc = "Field `EXTI7_SS` writer - EXTI 7 sources selection"]
pub type EXTI7_SS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTI 4 sources selection"]
    #[inline(always)]
    pub fn exti4_ss(&self) -> EXTI4_SS_R {
        EXTI4_SS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 5 sources selection"]
    #[inline(always)]
    pub fn exti5_ss(&self) -> EXTI5_SS_R {
        EXTI5_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 6 sources selection"]
    #[inline(always)]
    pub fn exti6_ss(&self) -> EXTI6_SS_R {
        EXTI6_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 7 sources selection"]
    #[inline(always)]
    pub fn exti7_ss(&self) -> EXTI7_SS_R {
        EXTI7_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 4 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti4_ss(&mut self) -> EXTI4_SS_W<EXTISS1_SPEC> {
        EXTI4_SS_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI 5 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti5_ss(&mut self) -> EXTI5_SS_W<EXTISS1_SPEC> {
        EXTI5_SS_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI 6 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti6_ss(&mut self) -> EXTI6_SS_W<EXTISS1_SPEC> {
        EXTI6_SS_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI 7 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti7_ss(&mut self) -> EXTI7_SS_W<EXTISS1_SPEC> {
        EXTI7_SS_W::new(self, 12)
    }
}
#[doc = "EXTI sources selection register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`extiss1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extiss1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTISS1_SPEC;
impl crate::RegisterSpec for EXTISS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extiss1::R`](R) reader structure"]
impl crate::Readable for EXTISS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extiss1::W`](W) writer structure"]
impl crate::Writable for EXTISS1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTISS1 to value 0"]
impl crate::Resettable for EXTISS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
