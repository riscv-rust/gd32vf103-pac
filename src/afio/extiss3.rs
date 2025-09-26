#[doc = "Register `EXTISS3` reader"]
pub type R = crate::R<EXTISS3_SPEC>;
#[doc = "Register `EXTISS3` writer"]
pub type W = crate::W<EXTISS3_SPEC>;
#[doc = "Field `EXTI12_SS` reader - EXTI 12 sources selection"]
pub type EXTI12_SS_R = crate::FieldReader;
#[doc = "Field `EXTI12_SS` writer - EXTI 12 sources selection"]
pub type EXTI12_SS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI13_SS` reader - EXTI 13 sources selection"]
pub type EXTI13_SS_R = crate::FieldReader;
#[doc = "Field `EXTI13_SS` writer - EXTI 13 sources selection"]
pub type EXTI13_SS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI14_SS` reader - EXTI 14 sources selection"]
pub type EXTI14_SS_R = crate::FieldReader;
#[doc = "Field `EXTI14_SS` writer - EXTI 14 sources selection"]
pub type EXTI14_SS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI15_SS` reader - EXTI 15 sources selection"]
pub type EXTI15_SS_R = crate::FieldReader;
#[doc = "Field `EXTI15_SS` writer - EXTI 15 sources selection"]
pub type EXTI15_SS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTI 12 sources selection"]
    #[inline(always)]
    pub fn exti12_ss(&self) -> EXTI12_SS_R {
        EXTI12_SS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 13 sources selection"]
    #[inline(always)]
    pub fn exti13_ss(&self) -> EXTI13_SS_R {
        EXTI13_SS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 14 sources selection"]
    #[inline(always)]
    pub fn exti14_ss(&self) -> EXTI14_SS_R {
        EXTI14_SS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 15 sources selection"]
    #[inline(always)]
    pub fn exti15_ss(&self) -> EXTI15_SS_R {
        EXTI15_SS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 12 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti12_ss(&mut self) -> EXTI12_SS_W<EXTISS3_SPEC> {
        EXTI12_SS_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI 13 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti13_ss(&mut self) -> EXTI13_SS_W<EXTISS3_SPEC> {
        EXTI13_SS_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI 14 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti14_ss(&mut self) -> EXTI14_SS_W<EXTISS3_SPEC> {
        EXTI14_SS_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI 15 sources selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti15_ss(&mut self) -> EXTI15_SS_W<EXTISS3_SPEC> {
        EXTI15_SS_W::new(self, 12)
    }
}
#[doc = "EXTI sources selection register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`extiss3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extiss3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTISS3_SPEC;
impl crate::RegisterSpec for EXTISS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extiss3::R`](R) reader structure"]
impl crate::Readable for EXTISS3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`extiss3::W`](W) writer structure"]
impl crate::Writable for EXTISS3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTISS3 to value 0"]
impl crate::Resettable for EXTISS3_SPEC {
    const RESET_VALUE: u32 = 0;
}
