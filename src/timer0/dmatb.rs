#[doc = "Register `DMATB` reader"]
pub type R = crate::R<DMATB_SPEC>;
#[doc = "Register `DMATB` writer"]
pub type W = crate::W<DMATB_SPEC>;
#[doc = "Field `DMATB` reader - DMA transfer buffer"]
pub type DMATB_R = crate::FieldReader<u16>;
#[doc = "Field `DMATB` writer - DMA transfer buffer"]
pub type DMATB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DMA transfer buffer"]
    #[inline(always)]
    pub fn dmatb(&self) -> DMATB_R {
        DMATB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA transfer buffer"]
    #[inline(always)]
    #[must_use]
    pub fn dmatb(&mut self) -> DMATB_W<DMATB_SPEC> {
        DMATB_W::new(self, 0)
    }
}
#[doc = "DMA transfer buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATB_SPEC;
impl crate::RegisterSpec for DMATB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmatb::R`](R) reader structure"]
impl crate::Readable for DMATB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmatb::W`](W) writer structure"]
impl crate::Writable for DMATB_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DMATB to value 0"]
impl crate::Resettable for DMATB_SPEC {
    const RESET_VALUE: u16 = 0;
}
