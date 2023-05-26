#[doc = "Register `HCH4CTL` reader"]
pub struct R(crate::R<HCH4CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCH4CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCH4CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCH4CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCH4CTL` writer"]
pub struct W(crate::W<HCH4CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCH4CTL_SPEC>;
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
impl From<crate::W<HCH4CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCH4CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPL` reader - Maximum packet size"]
pub type MPL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MPL` writer - Maximum packet size"]
pub type MPL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCH4CTL_SPEC, u16, u16, 11, O>;
#[doc = "Field `EPNUM` reader - Endpoint number"]
pub type EPNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPNUM` writer - Endpoint number"]
pub type EPNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCH4CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `EPDIR` reader - Endpoint direction"]
pub type EPDIR_R = crate::BitReader<bool>;
#[doc = "Field `EPDIR` writer - Endpoint direction"]
pub type EPDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH4CTL_SPEC, bool, O>;
#[doc = "Field `LSD` reader - Low-speed device"]
pub type LSD_R = crate::BitReader<bool>;
#[doc = "Field `LSD` writer - Low-speed device"]
pub type LSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH4CTL_SPEC, bool, O>;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EPTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPTYPE` writer - Endpoint type"]
pub type EPTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCH4CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DAR` reader - Device address"]
pub type DAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAR` writer - Device address"]
pub type DAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCH4CTL_SPEC, u8, u8, 7, O>;
#[doc = "Field `ODDFRM` reader - Odd frame"]
pub type ODDFRM_R = crate::BitReader<bool>;
#[doc = "Field `ODDFRM` writer - Odd frame"]
pub type ODDFRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH4CTL_SPEC, bool, O>;
#[doc = "Field `CDIS` reader - Channel disable"]
pub type CDIS_R = crate::BitReader<bool>;
#[doc = "Field `CDIS` writer - Channel disable"]
pub type CDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH4CTL_SPEC, bool, O>;
#[doc = "Field `CEN` reader - Channel enable"]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - Channel enable"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCH4CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mpl(&self) -> MPL_R {
        MPL_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    pub fn lsd(&self) -> LSD_R {
        LSD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    pub fn cdis(&self) -> CDIS_R {
        CDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    #[must_use]
    pub fn mpl(&mut self) -> MPL_W<0> {
        MPL_W::new(self)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EPNUM_W<11> {
        EPNUM_W::new(self)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<15> {
        EPDIR_W::new(self)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    #[must_use]
    pub fn lsd(&mut self) -> LSD_W<17> {
        LSD_W::new(self)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<18> {
        EPTYPE_W::new(self)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<22> {
        DAR_W::new(self)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    #[must_use]
    pub fn oddfrm(&mut self) -> ODDFRM_W<29> {
        ODDFRM_W::new(self)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    #[must_use]
    pub fn cdis(&mut self) -> CDIS_W<30> {
        CDIS_W::new(self)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<31> {
        CEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "host channel-4 characteristics register (HCH4CTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch4ctl](index.html) module"]
pub struct HCH4CTL_SPEC;
impl crate::RegisterSpec for HCH4CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hch4ctl::R](R) reader structure"]
impl crate::Readable for HCH4CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hch4ctl::W](W) writer structure"]
impl crate::Writable for HCH4CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCH4CTL to value 0"]
impl crate::Resettable for HCH4CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
