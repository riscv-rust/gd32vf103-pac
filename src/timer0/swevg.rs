#[doc = "Register `SWEVG` writer"]
pub struct W(crate::W<SWEVG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWEVG_SPEC>;
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
impl From<crate::W<SWEVG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWEVG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPG` writer - Update event generation"]
pub type UPG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWEVG_SPEC, bool, O>;
#[doc = "Field `CH0G` writer - Channel 0 capture or compare event generation"]
pub type CH0G_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWEVG_SPEC, bool, O>;
#[doc = "Field `CH1G` writer - Channel 1 capture or compare event generation"]
pub type CH1G_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWEVG_SPEC, bool, O>;
#[doc = "Field `CH2G` writer - Channel 2 capture or compare event generation"]
pub type CH2G_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWEVG_SPEC, bool, O>;
#[doc = "Field `CH3G` writer - Channel 3 capture or compare event generation"]
pub type CH3G_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWEVG_SPEC, bool, O>;
#[doc = "Field `CMTG` writer - Channel commutation event generation"]
pub type CMTG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWEVG_SPEC, bool, O>;
#[doc = "Field `TRGG` writer - Trigger event generation"]
pub type TRGG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWEVG_SPEC, bool, O>;
#[doc = "Field `BRKG` writer - Break event generation"]
pub type BRKG_W<'a, const O: u8> = crate::BitWriter<'a, u16, SWEVG_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Update event generation"]
    #[inline(always)]
    #[must_use]
    pub fn upg(&mut self) -> UPG_W<0> {
        UPG_W::new(self)
    }
    #[doc = "Bit 1 - Channel 0 capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch0g(&mut self) -> CH0G_W<1> {
        CH0G_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch1g(&mut self) -> CH1G_W<2> {
        CH1G_W::new(self)
    }
    #[doc = "Bit 3 - Channel 2 capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch2g(&mut self) -> CH2G_W<3> {
        CH2G_W::new(self)
    }
    #[doc = "Bit 4 - Channel 3 capture or compare event generation"]
    #[inline(always)]
    #[must_use]
    pub fn ch3g(&mut self) -> CH3G_W<4> {
        CH3G_W::new(self)
    }
    #[doc = "Bit 5 - Channel commutation event generation"]
    #[inline(always)]
    #[must_use]
    pub fn cmtg(&mut self) -> CMTG_W<5> {
        CMTG_W::new(self)
    }
    #[doc = "Bit 6 - Trigger event generation"]
    #[inline(always)]
    #[must_use]
    pub fn trgg(&mut self) -> TRGG_W<6> {
        TRGG_W::new(self)
    }
    #[doc = "Bit 7 - Break event generation"]
    #[inline(always)]
    #[must_use]
    pub fn brkg(&mut self) -> BRKG_W<7> {
        BRKG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software event generation register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swevg](index.html) module"]
pub struct SWEVG_SPEC;
impl crate::RegisterSpec for SWEVG_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [swevg::W](W) writer structure"]
impl crate::Writable for SWEVG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWEVG to value 0"]
impl crate::Resettable for SWEVG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
