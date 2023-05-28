#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKPH` reader - Clock Phase Selection"]
pub type CKPH_R = crate::BitReader<bool>;
#[doc = "Field `CKPH` writer - Clock Phase Selection"]
pub type CKPH_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL0_SPEC, bool, O>;
#[doc = "Field `CKPL` reader - Clock polarity Selection"]
pub type CKPL_R = crate::BitReader<bool>;
#[doc = "Field `CKPL` writer - Clock polarity Selection"]
pub type CKPL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL0_SPEC, bool, O>;
#[doc = "Field `MSTMOD` reader - Master Mode Enable"]
pub type MSTMOD_R = crate::BitReader<bool>;
#[doc = "Field `MSTMOD` writer - Master Mode Enable"]
pub type MSTMOD_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL0_SPEC, bool, O>;
#[doc = "Field `PSC` reader - Master Clock Prescaler Selection"]
pub type PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSC` writer - Master Clock Prescaler Selection"]
pub type PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CTL0_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPIEN` reader - SPI enable"]
pub type SPIEN_R = crate::BitReader<bool>;
#[doc = "Field `SPIEN` writer - SPI enable"]
pub type SPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL0_SPEC, bool, O>;
#[doc = "Field `LF` reader - LSB First Mode"]
pub type LF_R = crate::BitReader<bool>;
#[doc = "Field `LF` writer - LSB First Mode"]
pub type LF_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL0_SPEC, bool, O>;
#[doc = "Field `SWNSS` reader - NSS Pin Selection In NSS Software Mode"]
pub type SWNSS_R = crate::BitReader<bool>;
#[doc = "Field `SWNSS` writer - NSS Pin Selection In NSS Software Mode"]
pub type SWNSS_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL0_SPEC, bool, O>;
#[doc = "Field `SWNSSEN` reader - NSS Software Mode Selection"]
pub type SWNSSEN_R = crate::BitReader<bool>;
#[doc = "Field `SWNSSEN` writer - NSS Software Mode Selection"]
pub type SWNSSEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL0_SPEC, bool, O>;
#[doc = "Field `RO` reader - Receive only"]
pub type RO_R = crate::BitReader<bool>;
#[doc = "Field `RO` writer - Receive only"]
pub type RO_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL0_SPEC, bool, O>;
#[doc = "Field `FF16` reader - Data frame format"]
pub type FF16_R = crate::BitReader<bool>;
#[doc = "Field `FF16` writer - Data frame format"]
pub type FF16_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL0_SPEC, bool, O>;
#[doc = "Field `CRCNT` reader - CRC Next Transfer"]
pub type CRCNT_R = crate::BitReader<bool>;
#[doc = "Field `CRCNT` writer - CRC Next Transfer"]
pub type CRCNT_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL0_SPEC, bool, O>;
#[doc = "Field `CRCEN` reader - CRC Calculation Enable"]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - CRC Calculation Enable"]
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL0_SPEC, bool, O>;
#[doc = "Field `BDOEN` reader - Bidirectional Transmit output enable"]
pub type BDOEN_R = crate::BitReader<bool>;
#[doc = "Field `BDOEN` writer - Bidirectional Transmit output enable"]
pub type BDOEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL0_SPEC, bool, O>;
#[doc = "Field `BDEN` reader - Bidirectional enable"]
pub type BDEN_R = crate::BitReader<bool>;
#[doc = "Field `BDEN` writer - Bidirectional enable"]
pub type BDEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTL0_SPEC, bool, O>;
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
    pub fn ckph(&mut self) -> CKPH_W<0> {
        CKPH_W::new(self)
    }
    #[doc = "Bit 1 - Clock polarity Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ckpl(&mut self) -> CKPL_W<1> {
        CKPL_W::new(self)
    }
    #[doc = "Bit 2 - Master Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mstmod(&mut self) -> MSTMOD_W<2> {
        MSTMOD_W::new(self)
    }
    #[doc = "Bits 3:5 - Master Clock Prescaler Selection"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<3> {
        PSC_W::new(self)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<6> {
        SPIEN_W::new(self)
    }
    #[doc = "Bit 7 - LSB First Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lf(&mut self) -> LF_W<7> {
        LF_W::new(self)
    }
    #[doc = "Bit 8 - NSS Pin Selection In NSS Software Mode"]
    #[inline(always)]
    #[must_use]
    pub fn swnss(&mut self) -> SWNSS_W<8> {
        SWNSS_W::new(self)
    }
    #[doc = "Bit 9 - NSS Software Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn swnssen(&mut self) -> SWNSSEN_W<9> {
        SWNSSEN_W::new(self)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    #[must_use]
    pub fn ro(&mut self) -> RO_W<10> {
        RO_W::new(self)
    }
    #[doc = "Bit 11 - Data frame format"]
    #[inline(always)]
    #[must_use]
    pub fn ff16(&mut self) -> FF16_W<11> {
        FF16_W::new(self)
    }
    #[doc = "Bit 12 - CRC Next Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn crcnt(&mut self) -> CRCNT_W<12> {
        CRCNT_W::new(self)
    }
    #[doc = "Bit 13 - CRC Calculation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<13> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 14 - Bidirectional Transmit output enable"]
    #[inline(always)]
    #[must_use]
    pub fn bdoen(&mut self) -> BDOEN_W<14> {
        BDOEN_W::new(self)
    }
    #[doc = "Bit 15 - Bidirectional enable"]
    #[inline(always)]
    #[must_use]
    pub fn bden(&mut self) -> BDEN_W<15> {
        BDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
