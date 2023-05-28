#[doc = "Register `DCFG` reader"]
pub struct R(crate::R<DCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCFG` writer"]
pub struct W(crate::W<DCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCFG_SPEC>;
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
impl From<crate::W<DCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DS` reader - Device speed"]
pub type DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS` writer - Device speed"]
pub type DS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `NZLSOH` reader - Non-zero-length status OUT handshake"]
pub type NZLSOH_R = crate::BitReader<bool>;
#[doc = "Field `NZLSOH` writer - Non-zero-length status OUT handshake"]
pub type NZLSOH_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCFG_SPEC, bool, O>;
#[doc = "Field `DAR` reader - Device address"]
pub type DAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAR` writer - Device address"]
pub type DAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFG_SPEC, u8, u8, 7, O>;
#[doc = "Field `EOPFT` reader - end of periodic frame time"]
pub type EOPFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EOPFT` writer - end of periodic frame time"]
pub type EOPFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsoh(&self) -> NZLSOH_R {
        NZLSOH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - end of periodic frame time"]
    #[inline(always)]
    pub fn eopft(&self) -> EOPFT_R {
        EOPFT_R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<0> {
        DS_W::new(self)
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    #[must_use]
    pub fn nzlsoh(&mut self) -> NZLSOH_W<2> {
        NZLSOH_W::new(self)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<4> {
        DAR_W::new(self)
    }
    #[doc = "Bits 11:12 - end of periodic frame time"]
    #[inline(always)]
    #[must_use]
    pub fn eopft(&mut self) -> EOPFT_W<11> {
        EOPFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device configuration register (DCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg](index.html) module"]
pub struct DCFG_SPEC;
impl crate::RegisterSpec for DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcfg::R](R) reader structure"]
impl crate::Readable for DCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcfg::W](W) writer structure"]
impl crate::Writable for DCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCFG to value 0"]
impl crate::Resettable for DCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
