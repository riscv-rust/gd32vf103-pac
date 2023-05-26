#[doc = "Register `DIEP1CTL` reader"]
pub struct R(crate::R<DIEP1CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP1CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP1CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP1CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEP1CTL` writer"]
pub struct W(crate::W<DIEP1CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEP1CTL_SPEC>;
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
impl From<crate::W<DIEP1CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEP1CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPL` reader - maximum packet length"]
pub type MPL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MPL` writer - maximum packet length"]
pub type MPL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEP1CTL_SPEC, u16, u16, 11, O>;
#[doc = "Field `EPACT` reader - Endpoint active"]
pub type EPACT_R = crate::BitReader<bool>;
#[doc = "Field `EPACT` writer - Endpoint active"]
pub type EPACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP1CTL_SPEC, bool, O>;
#[doc = "Field `EOFRM_DPID` reader - EOFRM/DPID"]
pub type EOFRM_DPID_R = crate::BitReader<bool>;
#[doc = "Field `NAKS` reader - NAK status"]
pub type NAKS_R = crate::BitReader<bool>;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EPTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPTYPE` writer - Endpoint type"]
pub type EPTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEP1CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `STALL` reader - STALL handshake"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - STALL handshake"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP1CTL_SPEC, bool, O>;
#[doc = "Field `TXFNUM` reader - Tx FIFO number"]
pub type TXFNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFNUM` writer - Tx FIFO number"]
pub type TXFNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEP1CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP1CTL_SPEC, bool, O>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP1CTL_SPEC, bool, O>;
#[doc = "Field `SD0PID_SEVENFRM` writer - SD0PID/SEVNFRM"]
pub type SD0PID_SEVENFRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP1CTL_SPEC, bool, O>;
#[doc = "Field `SD1PID_SODDFRM` writer - Set DATA1 PID/Set odd frame"]
pub type SD1PID_SODDFRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP1CTL_SPEC, bool, O>;
#[doc = "Field `EPD` reader - Endpoint disable"]
pub type EPD_R = crate::BitReader<bool>;
#[doc = "Field `EPD` writer - Endpoint disable"]
pub type EPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP1CTL_SPEC, bool, O>;
#[doc = "Field `EPEN` reader - Endpoint enable"]
pub type EPEN_R = crate::BitReader<bool>;
#[doc = "Field `EPEN` writer - Endpoint enable"]
pub type EPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEP1CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10 - maximum packet length"]
    #[inline(always)]
    pub fn mpl(&self) -> MPL_R {
        MPL_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Endpoint active"]
    #[inline(always)]
    pub fn epact(&self) -> EPACT_R {
        EPACT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - EOFRM/DPID"]
    #[inline(always)]
    pub fn eofrm_dpid(&self) -> EOFRM_DPID_R {
        EOFRM_DPID_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK status"]
    #[inline(always)]
    pub fn naks(&self) -> NAKS_R {
        NAKS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - Tx FIFO number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    pub fn epd(&self) -> EPD_R {
        EPD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    pub fn epen(&self) -> EPEN_R {
        EPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - maximum packet length"]
    #[inline(always)]
    #[must_use]
    pub fn mpl(&mut self) -> MPL_W<0> {
        MPL_W::new(self)
    }
    #[doc = "Bit 15 - Endpoint active"]
    #[inline(always)]
    #[must_use]
    pub fn epact(&mut self) -> EPACT_W<15> {
        EPACT_W::new(self)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<18> {
        EPTYPE_W::new(self)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<21> {
        STALL_W::new(self)
    }
    #[doc = "Bits 22:25 - Tx FIFO number"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<22> {
        TXFNUM_W::new(self)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<26> {
        CNAK_W::new(self)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<27> {
        SNAK_W::new(self)
    }
    #[doc = "Bit 28 - SD0PID/SEVNFRM"]
    #[inline(always)]
    #[must_use]
    pub fn sd0pid_sevenfrm(&mut self) -> SD0PID_SEVENFRM_W<28> {
        SD0PID_SEVENFRM_W::new(self)
    }
    #[doc = "Bit 29 - Set DATA1 PID/Set odd frame"]
    #[inline(always)]
    #[must_use]
    pub fn sd1pid_soddfrm(&mut self) -> SD1PID_SODDFRM_W<29> {
        SD1PID_SODDFRM_W::new(self)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    #[must_use]
    pub fn epd(&mut self) -> EPD_W<30> {
        EPD_W::new(self)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen(&mut self) -> EPEN_W<31> {
        EPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device in endpoint-1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep1ctl](index.html) module"]
pub struct DIEP1CTL_SPEC;
impl crate::RegisterSpec for DIEP1CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep1ctl::R](R) reader structure"]
impl crate::Readable for DIEP1CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diep1ctl::W](W) writer structure"]
impl crate::Writable for DIEP1CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEP1CTL to value 0"]
impl crate::Resettable for DIEP1CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
