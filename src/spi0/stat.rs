#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<STAT_SPEC>;
#[doc = "Field `RBNE` reader - Receive Buffer Not Empty"]
pub type RBNE_R = crate::BitReader;
#[doc = "Field `TBE` reader - Transmit Buffer Empty"]
pub type TBE_R = crate::BitReader;
#[doc = "Field `I2SCH` reader - I2S channel side"]
pub type I2SCH_R = crate::BitReader;
#[doc = "Field `TXURERR` reader - Transmission underrun error bit"]
pub type TXURERR_R = crate::BitReader;
#[doc = "Field `CRCERR` reader - SPI CRC Error Bit"]
pub type CRCERR_R = crate::BitReader;
#[doc = "Field `CRCERR` writer - SPI CRC Error Bit"]
pub type CRCERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONFERR` reader - SPI Configuration error"]
pub type CONFERR_R = crate::BitReader;
#[doc = "Field `RXORERR` reader - Reception Overrun Error Bit"]
pub type RXORERR_R = crate::BitReader;
#[doc = "Field `TRANS` reader - Transmitting On-going Bit"]
pub type TRANS_R = crate::BitReader;
#[doc = "Field `FERR` reader - Format error"]
pub type FERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive Buffer Not Empty"]
    #[inline(always)]
    pub fn rbne(&self) -> RBNE_R {
        RBNE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Buffer Empty"]
    #[inline(always)]
    pub fn tbe(&self) -> TBE_R {
        TBE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2S channel side"]
    #[inline(always)]
    pub fn i2sch(&self) -> I2SCH_R {
        I2SCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission underrun error bit"]
    #[inline(always)]
    pub fn txurerr(&self) -> TXURERR_R {
        TXURERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI CRC Error Bit"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI Configuration error"]
    #[inline(always)]
    pub fn conferr(&self) -> CONFERR_R {
        CONFERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reception Overrun Error Bit"]
    #[inline(always)]
    pub fn rxorerr(&self) -> RXORERR_R {
        RXORERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitting On-going Bit"]
    #[inline(always)]
    pub fn trans(&self) -> TRANS_R {
        TRANS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Format error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - SPI CRC Error Bit"]
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CRCERR_W<STAT_SPEC> {
        CRCERR_W::new(self, 4)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets STAT to value 0x02"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: u16 = 0x02;
}
