#[doc = "Register `CTL0` reader"]
pub type R = crate::R<CTL0_SPEC>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<CTL0_SPEC>;
#[doc = "Field `CKPH` reader - Clock Phase Selection"]
pub type CKPH_R = crate::BitReader;
#[doc = "Field `CKPH` writer - Clock Phase Selection"]
pub type CKPH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPL` reader - Clock polarity Selection"]
pub type CKPL_R = crate::BitReader;
#[doc = "Field `CKPL` writer - Clock polarity Selection"]
pub type CKPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTMOD` reader - Master Mode Enable"]
pub type MSTMOD_R = crate::BitReader;
#[doc = "Field `MSTMOD` writer - Master Mode Enable"]
pub type MSTMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSC` reader - Master Clock Prescaler Selection"]
pub type PSC_R = crate::FieldReader;
#[doc = "Field `PSC` writer - Master Clock Prescaler Selection"]
pub type PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPIEN` reader - SPI enable"]
pub type SPIEN_R = crate::BitReader;
#[doc = "Field `SPIEN` writer - SPI enable"]
pub type SPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LF` reader - LSB First Mode"]
pub type LF_R = crate::BitReader;
#[doc = "Field `LF` writer - LSB First Mode"]
pub type LF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWNSS` reader - NSS Pin Selection In NSS Software Mode"]
pub type SWNSS_R = crate::BitReader;
#[doc = "Field `SWNSS` writer - NSS Pin Selection In NSS Software Mode"]
pub type SWNSS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWNSSEN` reader - NSS Software Mode Selection"]
pub type SWNSSEN_R = crate::BitReader;
#[doc = "Field `SWNSSEN` writer - NSS Software Mode Selection"]
pub type SWNSSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RO` reader - Receive only"]
pub type RO_R = crate::BitReader;
#[doc = "Field `RO` writer - Receive only"]
pub type RO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FF16` reader - Data frame format"]
pub type FF16_R = crate::BitReader;
#[doc = "Field `FF16` writer - Data frame format"]
pub type FF16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCNT` reader - CRC Next Transfer"]
pub type CRCNT_R = crate::BitReader;
#[doc = "Field `CRCNT` writer - CRC Next Transfer"]
pub type CRCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC Calculation Enable"]
pub type CRCEN_R = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC Calculation Enable"]
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDOEN` reader - Bidirectional Transmit output enable"]
pub type BDOEN_R = crate::BitReader;
#[doc = "Field `BDOEN` writer - Bidirectional Transmit output enable"]
pub type BDOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDEN` reader - Bidirectional enable"]
pub type BDEN_R = crate::BitReader;
#[doc = "Field `BDEN` writer - Bidirectional enable"]
pub type BDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clock Phase Selection"]
    #[inline(always)]
    pub fn ckph(&self) -> CKPH_R {
        CKPH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity Selection"]
    #[inline(always)]
    pub fn ckpl(&self) -> CKPL_R {
        CKPL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Mode Enable"]
    #[inline(always)]
    pub fn mstmod(&self) -> MSTMOD_R {
        MSTMOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Master Clock Prescaler Selection"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LSB First Mode"]
    #[inline(always)]
    pub fn lf(&self) -> LF_R {
        LF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NSS Pin Selection In NSS Software Mode"]
    #[inline(always)]
    pub fn swnss(&self) -> SWNSS_R {
        SWNSS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NSS Software Mode Selection"]
    #[inline(always)]
    pub fn swnssen(&self) -> SWNSSEN_R {
        SWNSSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn ro(&self) -> RO_R {
        RO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    pub fn ff16(&self) -> FF16_R {
        FF16_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC Next Transfer"]
    #[inline(always)]
    pub fn crcnt(&self) -> CRCNT_R {
        CRCNT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CRC Calculation Enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bidirectional Transmit output enable"]
    #[inline(always)]
    pub fn bdoen(&self) -> BDOEN_R {
        BDOEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bidirectional enable"]
    #[inline(always)]
    pub fn bden(&self) -> BDEN_R {
        BDEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Phase Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckph(&mut self) -> CKPH_W<CTL0_SPEC> {
        CKPH_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clock polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckpl(&mut self) -> CKPL_W<CTL0_SPEC> {
        CKPL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Master Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mstmod(&mut self) -> MSTMOD_W<CTL0_SPEC> {
        MSTMOD_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - Master Clock Prescaler Selection"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<CTL0_SPEC> {
        PSC_W::new(self, 3)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<CTL0_SPEC> {
        SPIEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - LSB First Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lf(&mut self) -> LF_W<CTL0_SPEC> {
        LF_W::new(self, 7)
    }
    #[doc = "Bit 8 - NSS Pin Selection In NSS Software Mode"]
    #[inline(always)]
    #[must_use]
    pub fn swnss(&mut self) -> SWNSS_W<CTL0_SPEC> {
        SWNSS_W::new(self, 8)
    }
    #[doc = "Bit 9 - NSS Software Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn swnssen(&mut self) -> SWNSSEN_W<CTL0_SPEC> {
        SWNSSEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    #[must_use]
    pub fn ro(&mut self) -> RO_W<CTL0_SPEC> {
        RO_W::new(self, 10)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    #[must_use]
    pub fn ff16(&mut self) -> FF16_W<CTL0_SPEC> {
        FF16_W::new(self, 11)
    }
    #[doc = "Bit 12 - CRC Next Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn crcnt(&mut self) -> CRCNT_W<CTL0_SPEC> {
        CRCNT_W::new(self, 12)
    }
    #[doc = "Bit 13 - CRC Calculation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<CTL0_SPEC> {
        CRCEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - Bidirectional Transmit output enable"]
    #[inline(always)]
    #[must_use]
    pub fn bdoen(&mut self) -> BDOEN_W<CTL0_SPEC> {
        BDOEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Bidirectional enable"]
    #[inline(always)]
    #[must_use]
    pub fn bden(&mut self) -> BDEN_W<CTL0_SPEC> {
        BDEN_W::new(self, 15)
    }
}
#[doc = "control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for CTL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: u16 = 0;
}
