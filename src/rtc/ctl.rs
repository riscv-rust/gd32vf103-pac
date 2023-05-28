#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCIF` reader - Sencond interrupt flag"]
pub type SCIF_R = crate::BitReader<bool>;
#[doc = "Field `SCIF` writer - Sencond interrupt flag"]
pub type SCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ALRMIF` reader - Alarm interrupt flag"]
pub type ALRMIF_R = crate::BitReader<bool>;
#[doc = "Field `ALRMIF` writer - Alarm interrupt flag"]
pub type ALRMIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `OVIF` reader - Overflow interrupt flag"]
pub type OVIF_R = crate::BitReader<bool>;
#[doc = "Field `OVIF` writer - Overflow interrupt flag"]
pub type OVIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `RSYNF` reader - Registers synchronized flag"]
pub type RSYNF_R = crate::BitReader<bool>;
#[doc = "Field `RSYNF` writer - Registers synchronized flag"]
pub type RSYNF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `CMF` reader - Configuration mode flag"]
pub type CMF_R = crate::BitReader<bool>;
#[doc = "Field `CMF` writer - Configuration mode flag"]
pub type CMF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `LWOFF` reader - Last write operation finished flag"]
pub type LWOFF_R = crate::BitReader<bool>;
#[doc = "Field `LWOFF` writer - Last write operation finished flag"]
pub type LWOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Sencond interrupt flag"]
    #[inline(always)]
    pub fn scif(&self) -> SCIF_R {
        SCIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm interrupt flag"]
    #[inline(always)]
    pub fn alrmif(&self) -> ALRMIF_R {
        ALRMIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovif(&self) -> OVIF_R {
        OVIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Registers synchronized flag"]
    #[inline(always)]
    pub fn rsynf(&self) -> RSYNF_R {
        RSYNF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configuration mode flag"]
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Last write operation finished flag"]
    #[inline(always)]
    pub fn lwoff(&self) -> LWOFF_R {
        LWOFF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sencond interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn scif(&mut self) -> SCIF_W<0> {
        SCIF_W::new(self)
    }
    #[doc = "Bit 1 - Alarm interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn alrmif(&mut self) -> ALRMIF_W<1> {
        ALRMIF_W::new(self)
    }
    #[doc = "Bit 2 - Overflow interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovif(&mut self) -> OVIF_W<2> {
        OVIF_W::new(self)
    }
    #[doc = "Bit 3 - Registers synchronized flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsynf(&mut self) -> RSYNF_W<3> {
        RSYNF_W::new(self)
    }
    #[doc = "Bit 4 - Configuration mode flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmf(&mut self) -> CMF_W<4> {
        CMF_W::new(self)
    }
    #[doc = "Bit 5 - Last write operation finished flag"]
    #[inline(always)]
    #[must_use]
    pub fn lwoff(&mut self) -> LWOFF_W<5> {
        LWOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0x20"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
