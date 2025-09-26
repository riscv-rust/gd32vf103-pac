#[doc = "Register `CTL1` reader"]
pub type R = crate::R<CTL1_SPEC>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<CTL1_SPEC>;
#[doc = "Field `DMAREN` reader - Rx buffer DMA enable"]
pub type DMAREN_R = crate::BitReader;
#[doc = "Field `DMAREN` writer - Rx buffer DMA enable"]
pub type DMAREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMATEN` reader - Transmit Buffer DMA Enable"]
pub type DMATEN_R = crate::BitReader;
#[doc = "Field `DMATEN` writer - Transmit Buffer DMA Enable"]
pub type DMATEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSDRV` reader - Drive NSS Output"]
pub type NSSDRV_R = crate::BitReader;
#[doc = "Field `NSSDRV` writer - Drive NSS Output"]
pub type NSSDRV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSP` reader - SPI NSS pulse mode enable"]
pub type NSSP_R = crate::BitReader;
#[doc = "Field `NSSP` writer - SPI NSS pulse mode enable"]
pub type NSSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMOD` reader - SPI TI mode enable"]
pub type TMOD_R = crate::BitReader;
#[doc = "Field `TMOD` writer - SPI TI mode enable"]
pub type TMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBNEIE` reader - RX buffer not empty interrupt enable"]
pub type RBNEIE_R = crate::BitReader;
#[doc = "Field `RBNEIE` writer - RX buffer not empty interrupt enable"]
pub type RBNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBEIE` reader - Tx buffer empty interrupt enable"]
pub type TBEIE_R = crate::BitReader;
#[doc = "Field `TBEIE` writer - Tx buffer empty interrupt enable"]
pub type TBEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Buffer DMA Enable"]
    #[inline(always)]
    pub fn dmaten(&self) -> DMATEN_R {
        DMATEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Drive NSS Output"]
    #[inline(always)]
    pub fn nssdrv(&self) -> NSSDRV_R {
        NSSDRV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI NSS pulse mode enable"]
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI TI mode enable"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rbneie(&self) -> RBNEIE_R {
        RBNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn tbeie(&self) -> TBEIE_R {
        TBEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaren(&mut self) -> DMAREN_W<CTL1_SPEC> {
        DMAREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Buffer DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaten(&mut self) -> DMATEN_W<CTL1_SPEC> {
        DMATEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Drive NSS Output"]
    #[inline(always)]
    #[must_use]
    pub fn nssdrv(&mut self) -> NSSDRV_W<CTL1_SPEC> {
        NSSDRV_W::new(self, 2)
    }
    #[doc = "Bit 3 - SPI NSS pulse mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn nssp(&mut self) -> NSSP_W<CTL1_SPEC> {
        NSSP_W::new(self, 3)
    }
    #[doc = "Bit 4 - SPI TI mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn tmod(&mut self) -> TMOD_W<CTL1_SPEC> {
        TMOD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTL1_SPEC> {
        ERRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbneie(&mut self) -> RBNEIE_W<CTL1_SPEC> {
        RBNEIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbeie(&mut self) -> TBEIE_W<CTL1_SPEC> {
        TBEIE_W::new(self, 7)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: u16 = 0;
}
