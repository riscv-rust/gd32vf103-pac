#[doc = "Register `GAHBCS` reader"]
pub type R = crate::R<GAHBCS_SPEC>;
#[doc = "Register `GAHBCS` writer"]
pub type W = crate::W<GAHBCS_SPEC>;
#[doc = "Field `GINTEN` reader - Global interrupt enable"]
pub type GINTEN_R = crate::BitReader;
#[doc = "Field `GINTEN` writer - Global interrupt enable"]
pub type GINTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFTH` reader - Tx FIFO threshold"]
pub type TXFTH_R = crate::BitReader;
#[doc = "Field `TXFTH` writer - Tx FIFO threshold"]
pub type TXFTH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFTH` reader - Periodic Tx FIFO threshold"]
pub type PTXFTH_R = crate::BitReader;
#[doc = "Field `PTXFTH` writer - Periodic Tx FIFO threshold"]
pub type PTXFTH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    pub fn ginten(&self) -> GINTEN_R {
        GINTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Tx FIFO threshold"]
    #[inline(always)]
    pub fn txfth(&self) -> TXFTH_R {
        TXFTH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periodic Tx FIFO threshold"]
    #[inline(always)]
    pub fn ptxfth(&self) -> PTXFTH_R {
        PTXFTH_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ginten(&mut self) -> GINTEN_W<GAHBCS_SPEC> {
        GINTEN_W::new(self, 0)
    }
    #[doc = "Bit 7 - Tx FIFO threshold"]
    #[inline(always)]
    #[must_use]
    pub fn txfth(&mut self) -> TXFTH_W<GAHBCS_SPEC> {
        TXFTH_W::new(self, 7)
    }
    #[doc = "Bit 8 - Periodic Tx FIFO threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfth(&mut self) -> PTXFTH_W<GAHBCS_SPEC> {
        PTXFTH_W::new(self, 8)
    }
}
#[doc = "Global AHB control and status register (USBFS_GAHBCS)\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAHBCS_SPEC;
impl crate::RegisterSpec for GAHBCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcs::R`](R) reader structure"]
impl crate::Readable for GAHBCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gahbcs::W`](W) writer structure"]
impl crate::Writable for GAHBCS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAHBCS to value 0"]
impl crate::Resettable for GAHBCS_SPEC {
    const RESET_VALUE: u32 = 0;
}
