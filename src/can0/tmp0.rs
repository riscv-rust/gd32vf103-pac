#[doc = "Register `TMP0` reader"]
pub struct R(crate::R<TMP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMP0` writer"]
pub struct W(crate::W<TMP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMP0_SPEC>;
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
impl From<crate::W<TMP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLENC` reader - Data length code"]
pub type DLENC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLENC` writer - Data length code"]
pub type DLENC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMP0_SPEC, u8, u8, 4, O>;
#[doc = "Field `TSEN` reader - Time stamp enable"]
pub type TSEN_R = crate::BitReader<bool>;
#[doc = "Field `TSEN` writer - Time stamp enable"]
pub type TSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMP0_SPEC, bool, O>;
#[doc = "Field `TS` reader - Time stamp"]
pub type TS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TS` writer - Time stamp"]
pub type TS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TMP0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlenc(&self) -> DLENC_R {
        DLENC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Time stamp enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Time stamp"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    #[must_use]
    pub fn dlenc(&mut self) -> DLENC_W<0> {
        DLENC_W::new(self)
    }
    #[doc = "Bit 8 - Time stamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<8> {
        TSEN_W::new(self)
    }
    #[doc = "Bits 16:31 - Time stamp"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<16> {
        TS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit mailbox property register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmp0](index.html) module"]
pub struct TMP0_SPEC;
impl crate::RegisterSpec for TMP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmp0::R](R) reader structure"]
impl crate::Readable for TMP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmp0::W](W) writer structure"]
impl crate::Writable for TMP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMP0 to value 0"]
impl crate::Resettable for TMP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
