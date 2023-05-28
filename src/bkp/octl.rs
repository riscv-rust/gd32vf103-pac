#[doc = "Register `OCTL` reader"]
pub struct R(crate::R<OCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTL` writer"]
pub struct W(crate::W<OCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTL_SPEC>;
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
impl From<crate::W<OCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCCV` reader - RTC clock calibration value"]
pub type RCCV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCCV` writer - RTC clock calibration value"]
pub type RCCV_W<'a, const O: u8> = crate::FieldWriter<'a, u16, OCTL_SPEC, u8, u8, 7, O>;
#[doc = "Field `COEN` reader - RTC clock calibration output enable"]
pub type COEN_R = crate::BitReader<bool>;
#[doc = "Field `COEN` writer - RTC clock calibration output enable"]
pub type COEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, OCTL_SPEC, bool, O>;
#[doc = "Field `ASOEN` reader - RTC alarm or second signal output enable"]
pub type ASOEN_R = crate::BitReader<bool>;
#[doc = "Field `ASOEN` writer - RTC alarm or second signal output enable"]
pub type ASOEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, OCTL_SPEC, bool, O>;
#[doc = "Field `ROSEL` reader - RTC output selection"]
pub type ROSEL_R = crate::BitReader<bool>;
#[doc = "Field `ROSEL` writer - RTC output selection"]
pub type ROSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, OCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - RTC clock calibration value"]
    #[inline(always)]
    pub fn rccv(&self) -> RCCV_R {
        RCCV_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - RTC clock calibration output enable"]
    #[inline(always)]
    pub fn coen(&self) -> COEN_R {
        COEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC alarm or second signal output enable"]
    #[inline(always)]
    pub fn asoen(&self) -> ASOEN_R {
        ASOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RTC output selection"]
    #[inline(always)]
    pub fn rosel(&self) -> ROSEL_R {
        ROSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - RTC clock calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn rccv(&mut self) -> RCCV_W<0> {
        RCCV_W::new(self)
    }
    #[doc = "Bit 7 - RTC clock calibration output enable"]
    #[inline(always)]
    #[must_use]
    pub fn coen(&mut self) -> COEN_W<7> {
        COEN_W::new(self)
    }
    #[doc = "Bit 8 - RTC alarm or second signal output enable"]
    #[inline(always)]
    #[must_use]
    pub fn asoen(&mut self) -> ASOEN_W<8> {
        ASOEN_W::new(self)
    }
    #[doc = "Bit 9 - RTC output selection"]
    #[inline(always)]
    #[must_use]
    pub fn rosel(&mut self) -> ROSEL_W<9> {
        ROSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC signal output control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octl](index.html) module"]
pub struct OCTL_SPEC;
impl crate::RegisterSpec for OCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [octl::R](R) reader structure"]
impl crate::Readable for OCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octl::W](W) writer structure"]
impl crate::Writable for OCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OCTL to value 0"]
impl crate::Resettable for OCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
