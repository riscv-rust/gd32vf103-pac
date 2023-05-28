#[doc = "Register `DOEP3INTF` reader"]
pub struct R(crate::R<DOEP3INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEP3INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEP3INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEP3INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEP3INTF` writer"]
pub struct W(crate::W<DOEP3INTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEP3INTF_SPEC>;
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
impl From<crate::W<DOEP3INTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEP3INTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TF` reader - Transfer finished"]
pub type TF_R = crate::BitReader<bool>;
#[doc = "Field `TF` writer - Transfer finished"]
pub type TF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP3INTF_SPEC, bool, O>;
#[doc = "Field `EPDIS` reader - Endpoint disabled"]
pub type EPDIS_R = crate::BitReader<bool>;
#[doc = "Field `EPDIS` writer - Endpoint disabled"]
pub type EPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP3INTF_SPEC, bool, O>;
#[doc = "Field `STPF` reader - Setup phase finished"]
pub type STPF_R = crate::BitReader<bool>;
#[doc = "Field `STPF` writer - Setup phase finished"]
pub type STPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP3INTF_SPEC, bool, O>;
#[doc = "Field `EPRXFOVR` reader - Endpoint Rx FIFO overrun"]
pub type EPRXFOVR_R = crate::BitReader<bool>;
#[doc = "Field `EPRXFOVR` writer - Endpoint Rx FIFO overrun"]
pub type EPRXFOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP3INTF_SPEC, bool, O>;
#[doc = "Field `BTBSTP` reader - Back-to-back SETUP packets"]
pub type BTBSTP_R = crate::BitReader<bool>;
#[doc = "Field `BTBSTP` writer - Back-to-back SETUP packets"]
pub type BTBSTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP3INTF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    pub fn tf(&self) -> TF_R {
        TF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Setup phase finished"]
    #[inline(always)]
    pub fn stpf(&self) -> STPF_R {
        STPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun"]
    #[inline(always)]
    pub fn eprxfovr(&self) -> EPRXFOVR_R {
        EPRXFOVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets"]
    #[inline(always)]
    pub fn btbstp(&self) -> BTBSTP_R {
        BTBSTP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer finished"]
    #[inline(always)]
    #[must_use]
    pub fn tf(&mut self) -> TF_W<0> {
        TF_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled"]
    #[inline(always)]
    #[must_use]
    pub fn epdis(&mut self) -> EPDIS_W<1> {
        EPDIS_W::new(self)
    }
    #[doc = "Bit 3 - Setup phase finished"]
    #[inline(always)]
    #[must_use]
    pub fn stpf(&mut self) -> STPF_W<3> {
        STPF_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint Rx FIFO overrun"]
    #[inline(always)]
    #[must_use]
    pub fn eprxfovr(&mut self) -> EPRXFOVR_W<4> {
        EPRXFOVR_W::new(self)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets"]
    #[inline(always)]
    #[must_use]
    pub fn btbstp(&mut self) -> BTBSTP_W<6> {
        BTBSTP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device out endpoint-3 interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep3intf](index.html) module"]
pub struct DOEP3INTF_SPEC;
impl crate::RegisterSpec for DOEP3INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doep3intf::R](R) reader structure"]
impl crate::Readable for DOEP3INTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doep3intf::W](W) writer structure"]
impl crate::Writable for DOEP3INTF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEP3INTF to value 0"]
impl crate::Resettable for DOEP3INTF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
