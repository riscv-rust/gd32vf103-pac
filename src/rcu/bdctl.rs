#[doc = "Register `BDCTL` reader"]
pub struct R(crate::R<BDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDCTL` writer"]
pub struct W(crate::W<BDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDCTL_SPEC>;
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
impl From<crate::W<BDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LXTALEN` reader - LXTAL enable"]
pub type LXTALEN_R = crate::BitReader<bool>;
#[doc = "Field `LXTALEN` writer - LXTAL enable"]
pub type LXTALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCTL_SPEC, bool, O>;
#[doc = "Field `LXTALSTB` reader - External low-speed oscillator stabilization"]
pub type LXTALSTB_R = crate::BitReader<bool>;
#[doc = "Field `LXTALBPS` reader - LXTAL bypass mode enable"]
pub type LXTALBPS_R = crate::BitReader<bool>;
#[doc = "Field `LXTALBPS` writer - LXTAL bypass mode enable"]
pub type LXTALBPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCTL_SPEC, bool, O>;
#[doc = "Field `RTCSRC` reader - RTC clock entry selection"]
pub type RTCSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCSRC` writer - RTC clock entry selection"]
pub type RTCSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RTCEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCTL_SPEC, bool, O>;
#[doc = "Field `BKPRST` reader - Backup domain reset"]
pub type BKPRST_R = crate::BitReader<bool>;
#[doc = "Field `BKPRST` writer - Backup domain reset"]
pub type BKPRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LXTAL enable"]
    #[inline(always)]
    pub fn lxtalen(&self) -> LXTALEN_R {
        LXTALEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External low-speed oscillator stabilization"]
    #[inline(always)]
    pub fn lxtalstb(&self) -> LXTALSTB_R {
        LXTALSTB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LXTAL bypass mode enable"]
    #[inline(always)]
    pub fn lxtalbps(&self) -> LXTALBPS_R {
        LXTALBPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock entry selection"]
    #[inline(always)]
    pub fn rtcsrc(&self) -> RTCSRC_R {
        RTCSRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup domain reset"]
    #[inline(always)]
    pub fn bkprst(&self) -> BKPRST_R {
        BKPRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LXTAL enable"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalen(&mut self) -> LXTALEN_W<0> {
        LXTALEN_W::new(self)
    }
    #[doc = "Bit 2 - LXTAL bypass mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lxtalbps(&mut self) -> LXTALBPS_W<2> {
        LXTALBPS_W::new(self)
    }
    #[doc = "Bits 8:9 - RTC clock entry selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsrc(&mut self) -> RTCSRC_W<8> {
        RTCSRC_W::new(self)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<15> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 16 - Backup domain reset"]
    #[inline(always)]
    #[must_use]
    pub fn bkprst(&mut self) -> BKPRST_W<16> {
        BKPRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup domain control register (RCU_BDCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdctl](index.html) module"]
pub struct BDCTL_SPEC;
impl crate::RegisterSpec for BDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdctl::R](R) reader structure"]
impl crate::Readable for BDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdctl::W](W) writer structure"]
impl crate::Writable for BDCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDCTL to value 0x18"]
impl crate::Resettable for BDCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x18;
}
