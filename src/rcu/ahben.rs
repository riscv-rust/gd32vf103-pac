#[doc = "Register `AHBEN` reader"]
pub type R = crate::R<AHBEN_SPEC>;
#[doc = "Register `AHBEN` writer"]
pub type W = crate::W<AHBEN_SPEC>;
#[doc = "Field `DMA0EN` reader - DMA0 clock enable"]
pub type DMA0EN_R = crate::BitReader;
#[doc = "Field `DMA0EN` writer - DMA0 clock enable"]
pub type DMA0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type DMA1EN_R = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type DMA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMSPEN` reader - SRAM interface clock enable when sleep mode"]
pub type SRAMSPEN_R = crate::BitReader;
#[doc = "Field `SRAMSPEN` writer - SRAM interface clock enable when sleep mode"]
pub type SRAMSPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCSPEN` reader - FMC clock enable when sleep mode"]
pub type FMCSPEN_R = crate::BitReader;
#[doc = "Field `FMCSPEN` writer - FMC clock enable when sleep mode"]
pub type FMCSPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXMCEN` reader - EXMC clock enable"]
pub type EXMCEN_R = crate::BitReader;
#[doc = "Field `EXMCEN` writer - EXMC clock enable"]
pub type EXMCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBFSEN` reader - USBFS clock enable"]
pub type USBFSEN_R = crate::BitReader;
#[doc = "Field `USBFSEN` writer - USBFS clock enable"]
pub type USBFSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA0 clock enable"]
    #[inline(always)]
    pub fn dma0en(&self) -> DMA0EN_R {
        DMA0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable when sleep mode"]
    #[inline(always)]
    pub fn sramspen(&self) -> SRAMSPEN_R {
        SRAMSPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FMC clock enable when sleep mode"]
    #[inline(always)]
    pub fn fmcspen(&self) -> FMCSPEN_R {
        FMCSPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - EXMC clock enable"]
    #[inline(always)]
    pub fn exmcen(&self) -> EXMCEN_R {
        EXMCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - USBFS clock enable"]
    #[inline(always)]
    pub fn usbfsen(&self) -> USBFSEN_R {
        USBFSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA0 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma0en(&mut self) -> DMA0EN_W<AHBEN_SPEC> {
        DMA0EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<AHBEN_SPEC> {
        DMA1EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - SRAM interface clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sramspen(&mut self) -> SRAMSPEN_W<AHBEN_SPEC> {
        SRAMSPEN_W::new(self, 2)
    }
    #[doc = "Bit 4 - FMC clock enable when sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn fmcspen(&mut self) -> FMCSPEN_W<AHBEN_SPEC> {
        FMCSPEN_W::new(self, 4)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<AHBEN_SPEC> {
        CRCEN_W::new(self, 6)
    }
    #[doc = "Bit 8 - EXMC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn exmcen(&mut self) -> EXMCEN_W<AHBEN_SPEC> {
        EXMCEN_W::new(self, 8)
    }
    #[doc = "Bit 12 - USBFS clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbfsen(&mut self) -> USBFSEN_W<AHBEN_SPEC> {
        USBFSEN_W::new(self, 12)
    }
}
#[doc = "AHB enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahben::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahben::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBEN_SPEC;
impl crate::RegisterSpec for AHBEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben::R`](R) reader structure"]
impl crate::Readable for AHBEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahben::W`](W) writer structure"]
impl crate::Writable for AHBEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBEN to value 0x14"]
impl crate::Resettable for AHBEN_SPEC {
    const RESET_VALUE: u32 = 0x14;
}
