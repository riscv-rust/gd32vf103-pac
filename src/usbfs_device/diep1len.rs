#[doc = "Register `DIEP1LEN` reader"]
pub struct R(crate::R<DIEP1LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP1LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP1LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP1LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEP1LEN` writer"]
pub struct W(crate::W<DIEP1LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEP1LEN_SPEC>;
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
impl From<crate::W<DIEP1LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEP1LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLEN` reader - Transfer length"]
pub type TLEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TLEN` writer - Transfer length"]
pub type TLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEP1LEN_SPEC, u32, u32, 19, O>;
#[doc = "Field `PCNT` reader - Packet count"]
pub type PCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCNT` writer - Packet count"]
pub type PCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEP1LEN_SPEC, u16, u16, 10, O>;
#[doc = "Field `MCPF` reader - Multi packet count per frame"]
pub type MCPF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCPF` writer - Multi packet count per frame"]
pub type MCPF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEP1LEN_SPEC, u8, u8, 2, O>;
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
    #[doc = "Bits 29:30 - Multi packet count per frame"]
    #[inline(always)]
    pub fn mcpf(&self) -> MCPF_R {
        MCPF_R::new(((self.bits >> 29) & 3) as u8)
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
    #[doc = "Bits 29:30 - Multi packet count per frame"]
    #[inline(always)]
    #[must_use]
    pub fn mcpf(&mut self) -> MCPF_W<29> {
        MCPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device IN endpoint-1 transfer length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep1len](index.html) module"]
pub struct DIEP1LEN_SPEC;
impl crate::RegisterSpec for DIEP1LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep1len::R](R) reader structure"]
impl crate::Readable for DIEP1LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diep1len::W](W) writer structure"]
impl crate::Writable for DIEP1LEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEP1LEN to value 0"]
impl crate::Resettable for DIEP1LEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
