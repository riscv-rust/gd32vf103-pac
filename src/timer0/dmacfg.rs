#[doc = "Register `DMACFG` reader"]
pub type R = crate::R<DMACFG_SPEC>;
#[doc = "Register `DMACFG` writer"]
pub type W = crate::W<DMACFG_SPEC>;
#[doc = "Field `DMATA` reader - DMA transfer access start address"]
pub type DMATA_R = crate::FieldReader;
#[doc = "Field `DMATA` writer - DMA transfer access start address"]
pub type DMATA_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DMATC` reader - DMA transfer count"]
pub type DMATC_R = crate::FieldReader;
#[doc = "Field `DMATC` writer - DMA transfer count"]
pub type DMATC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DMA transfer access start address"]
    #[inline(always)]
    pub fn dmata(&self) -> DMATA_R {
        DMATA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA transfer count"]
    #[inline(always)]
    pub fn dmatc(&self) -> DMATC_R {
        DMATC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA transfer access start address"]
    #[inline(always)]
    #[must_use]
    pub fn dmata(&mut self) -> DMATA_W<DMACFG_SPEC> {
        DMATA_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA transfer count"]
    #[inline(always)]
    #[must_use]
    pub fn dmatc(&mut self) -> DMATC_W<DMACFG_SPEC> {
        DMATC_W::new(self, 8)
    }
}
#[doc = "DMA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACFG_SPEC;
impl crate::RegisterSpec for DMACFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dmacfg::R`](R) reader structure"]
impl crate::Readable for DMACFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmacfg::W`](W) writer structure"]
impl crate::Writable for DMACFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DMACFG to value 0"]
impl crate::Resettable for DMACFG_SPEC {
    const RESET_VALUE: u16 = 0;
}
