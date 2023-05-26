#[doc = "Register `HCH6LEN` reader"]
pub struct R(crate::R<HCH6LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCH6LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCH6LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCH6LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCH6LEN` writer"]
pub struct W(crate::W<HCH6LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCH6LEN_SPEC>;
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
impl From<crate::W<HCH6LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCH6LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLEN` reader - Transfer length"]
pub type TLEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TLEN` writer - Transfer length"]
pub type TLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCH6LEN_SPEC, u32, u32, 19, O>;
#[doc = "Field `PCNT` reader - Packet count"]
pub type PCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCNT` writer - Packet count"]
pub type PCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCH6LEN_SPEC, u16, u16, 10, O>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DPID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DPID` writer - Data PID"]
pub type DPID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCH6LEN_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pcnt(&self) -> PCNT_R {
        PCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer length"]
    #[inline(always)]
    #[must_use]
    pub fn tlen(&mut self) -> TLEN_W<0> {
        TLEN_W::new(self)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt(&mut self) -> PCNT_W<19> {
        PCNT_W::new(self)
    }
    #[doc = "Bits 29:30 - Data PID"]
    #[inline(always)]
    #[must_use]
    pub fn dpid(&mut self) -> DPID_W<29> {
        DPID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "host channel-6 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hch6len](index.html) module"]
pub struct HCH6LEN_SPEC;
impl crate::RegisterSpec for HCH6LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hch6len::R](R) reader structure"]
impl crate::Readable for HCH6LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hch6len::W](W) writer structure"]
impl crate::Writable for HCH6LEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCH6LEN to value 0"]
impl crate::Resettable for HCH6LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
