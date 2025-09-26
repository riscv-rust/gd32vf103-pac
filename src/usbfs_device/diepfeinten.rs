#[doc = "Register `DIEPFEINTEN` reader"]
pub type R = crate::R<DIEPFEINTEN_SPEC>;
#[doc = "Register `DIEPFEINTEN` writer"]
pub type W = crate::W<DIEPFEINTEN_SPEC>;
#[doc = "Field `IEPTXFEIE` reader - IN EP Tx FIFO empty interrupt enable bits"]
pub type IEPTXFEIE_R = crate::FieldReader;
#[doc = "Field `IEPTXFEIE` writer - IN EP Tx FIFO empty interrupt enable bits"]
pub type IEPTXFEIE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - IN EP Tx FIFO empty interrupt enable bits"]
    #[inline(always)]
    pub fn ieptxfeie(&self) -> IEPTXFEIE_R {
        IEPTXFEIE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IN EP Tx FIFO empty interrupt enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn ieptxfeie(&mut self) -> IEPTXFEIE_W<DIEPFEINTEN_SPEC> {
        IEPTXFEIE_W::new(self, 0)
    }
}
#[doc = "device IN endpoint FIFO empty interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepfeinten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepfeinten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPFEINTEN_SPEC;
impl crate::RegisterSpec for DIEPFEINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepfeinten::R`](R) reader structure"]
impl crate::Readable for DIEPFEINTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepfeinten::W`](W) writer structure"]
impl crate::Writable for DIEPFEINTEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPFEINTEN to value 0"]
impl crate::Resettable for DIEPFEINTEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
