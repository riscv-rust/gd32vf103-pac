#[doc = "Register `DAEPINTEN` reader"]
pub type R = crate::R<DAEPINTEN_SPEC>;
#[doc = "Register `DAEPINTEN` writer"]
pub type W = crate::W<DAEPINTEN_SPEC>;
#[doc = "Field `IEPIE` reader - IN EP interrupt interrupt enable bits"]
pub type IEPIE_R = crate::FieldReader;
#[doc = "Field `IEPIE` writer - IN EP interrupt interrupt enable bits"]
pub type IEPIE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OEPIE` reader - OUT endpoint interrupt enable bits"]
pub type OEPIE_R = crate::FieldReader;
#[doc = "Field `OEPIE` writer - OUT endpoint interrupt enable bits"]
pub type OEPIE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - IN EP interrupt interrupt enable bits"]
    #[inline(always)]
    pub fn iepie(&self) -> IEPIE_R {
        IEPIE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - OUT endpoint interrupt enable bits"]
    #[inline(always)]
    pub fn oepie(&self) -> OEPIE_R {
        OEPIE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IN EP interrupt interrupt enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn iepie(&mut self) -> IEPIE_W<DAEPINTEN_SPEC> {
        IEPIE_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - OUT endpoint interrupt enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn oepie(&mut self) -> OEPIE_W<DAEPINTEN_SPEC> {
        OEPIE_W::new(self, 16)
    }
}
#[doc = "Device all endpoints interrupt enable register (DAEPINTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`daepinten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daepinten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAEPINTEN_SPEC;
impl crate::RegisterSpec for DAEPINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daepinten::R`](R) reader structure"]
impl crate::Readable for DAEPINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`daepinten::W`](W) writer structure"]
impl crate::Writable for DAEPINTEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAEPINTEN to value 0"]
impl crate::Resettable for DAEPINTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
