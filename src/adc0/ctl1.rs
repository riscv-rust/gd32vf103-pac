#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCON` reader - ADC on"]
pub type ADCON_R = crate::BitReader<bool>;
#[doc = "Field `ADCON` writer - ADC on"]
pub type ADCON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `CTN` reader - Continuous mode"]
pub type CTN_R = crate::BitReader<bool>;
#[doc = "Field `CTN` writer - Continuous mode"]
pub type CTN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `CLB` reader - ADC calibration"]
pub type CLB_R = crate::BitReader<bool>;
#[doc = "Field `CLB` writer - ADC calibration"]
pub type CLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `RSTCLB` reader - Reset calibration"]
pub type RSTCLB_R = crate::BitReader<bool>;
#[doc = "Field `RSTCLB` writer - Reset calibration"]
pub type RSTCLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `DMA` reader - DMA request enable"]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - DMA request enable"]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `DAL` reader - Data alignment"]
pub type DAL_R = crate::BitReader<bool>;
#[doc = "Field `DAL` writer - Data alignment"]
pub type DAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `ETSIC` reader - External trigger select for inserted channel"]
pub type ETSIC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETSIC` writer - External trigger select for inserted channel"]
pub type ETSIC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `ETEIC` reader - External trigger select for inserted channel"]
pub type ETEIC_R = crate::BitReader<bool>;
#[doc = "Field `ETEIC` writer - External trigger select for inserted channel"]
pub type ETEIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `ETSRC` reader - External trigger select for regular channel"]
pub type ETSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETSRC` writer - External trigger select for regular channel"]
pub type ETSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `ETERC` reader - External trigger enable for regular channel"]
pub type ETERC_R = crate::BitReader<bool>;
#[doc = "Field `ETERC` writer - External trigger enable for regular channel"]
pub type ETERC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `SWICST` reader - Start on inserted channel"]
pub type SWICST_R = crate::BitReader<bool>;
#[doc = "Field `SWICST` writer - Start on inserted channel"]
pub type SWICST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `SWRCST` reader - Start on regular channel"]
pub type SWRCST_R = crate::BitReader<bool>;
#[doc = "Field `SWRCST` writer - Start on regular channel"]
pub type SWRCST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `TSVREN` reader - Channel 16 and 17 enable of ADC0"]
pub type TSVREN_R = crate::BitReader<bool>;
#[doc = "Field `TSVREN` writer - Channel 16 and 17 enable of ADC0"]
pub type TSVREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ADC on"]
    #[inline(always)]
    pub fn adcon(&self) -> ADCON_R {
        ADCON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    pub fn ctn(&self) -> CTN_R {
        CTN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC calibration"]
    #[inline(always)]
    pub fn clb(&self) -> CLB_R {
        CLB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    pub fn rstclb(&self) -> RSTCLB_R {
        RSTCLB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    pub fn dal(&self) -> DAL_R {
        DAL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - External trigger select for inserted channel"]
    #[inline(always)]
    pub fn etsic(&self) -> ETSIC_R {
        ETSIC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - External trigger select for inserted channel"]
    #[inline(always)]
    pub fn eteic(&self) -> ETEIC_R {
        ETEIC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - External trigger select for regular channel"]
    #[inline(always)]
    pub fn etsrc(&self) -> ETSRC_R {
        ETSRC_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - External trigger enable for regular channel"]
    #[inline(always)]
    pub fn eterc(&self) -> ETERC_R {
        ETERC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Start on inserted channel"]
    #[inline(always)]
    pub fn swicst(&self) -> SWICST_R {
        SWICST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Start on regular channel"]
    #[inline(always)]
    pub fn swrcst(&self) -> SWRCST_R {
        SWRCST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 16 and 17 enable of ADC0"]
    #[inline(always)]
    pub fn tsvren(&self) -> TSVREN_R {
        TSVREN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC on"]
    #[inline(always)]
    #[must_use]
    pub fn adcon(&mut self) -> ADCON_W<0> {
        ADCON_W::new(self)
    }
    #[doc = "Bit 1 - Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn ctn(&mut self) -> CTN_W<1> {
        CTN_W::new(self)
    }
    #[doc = "Bit 2 - ADC calibration"]
    #[inline(always)]
    #[must_use]
    pub fn clb(&mut self) -> CLB_W<2> {
        CLB_W::new(self)
    }
    #[doc = "Bit 3 - Reset calibration"]
    #[inline(always)]
    #[must_use]
    pub fn rstclb(&mut self) -> RSTCLB_W<3> {
        RSTCLB_W::new(self)
    }
    #[doc = "Bit 8 - DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<8> {
        DMA_W::new(self)
    }
    #[doc = "Bit 11 - Data alignment"]
    #[inline(always)]
    #[must_use]
    pub fn dal(&mut self) -> DAL_W<11> {
        DAL_W::new(self)
    }
    #[doc = "Bits 12:14 - External trigger select for inserted channel"]
    #[inline(always)]
    #[must_use]
    pub fn etsic(&mut self) -> ETSIC_W<12> {
        ETSIC_W::new(self)
    }
    #[doc = "Bit 15 - External trigger select for inserted channel"]
    #[inline(always)]
    #[must_use]
    pub fn eteic(&mut self) -> ETEIC_W<15> {
        ETEIC_W::new(self)
    }
    #[doc = "Bits 17:19 - External trigger select for regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn etsrc(&mut self) -> ETSRC_W<17> {
        ETSRC_W::new(self)
    }
    #[doc = "Bit 20 - External trigger enable for regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn eterc(&mut self) -> ETERC_W<20> {
        ETERC_W::new(self)
    }
    #[doc = "Bit 21 - Start on inserted channel"]
    #[inline(always)]
    #[must_use]
    pub fn swicst(&mut self) -> SWICST_W<21> {
        SWICST_W::new(self)
    }
    #[doc = "Bit 22 - Start on regular channel"]
    #[inline(always)]
    #[must_use]
    pub fn swrcst(&mut self) -> SWRCST_W<22> {
        SWRCST_W::new(self)
    }
    #[doc = "Bit 23 - Channel 16 and 17 enable of ADC0"]
    #[inline(always)]
    #[must_use]
    pub fn tsvren(&mut self) -> TSVREN_W<23> {
        TSVREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
