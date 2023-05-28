#[doc = "Register `SNTCFG0` reader"]
pub struct R(crate::R<SNTCFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNTCFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNTCFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNTCFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNTCFG0` writer"]
pub struct W(crate::W<SNTCFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNTCFG0_SPEC>;
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
impl From<crate::W<SNTCFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNTCFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASET` reader - Address setup time"]
pub type ASET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ASET` writer - Address setup time"]
pub type ASET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SNTCFG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `AHLD` reader - Address hold time"]
pub type AHLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHLD` writer - Address hold time"]
pub type AHLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SNTCFG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `DSET` reader - Data setup time"]
pub type DSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSET` writer - Data setup time"]
pub type DSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SNTCFG0_SPEC, u8, u8, 8, O>;
#[doc = "Field `BUSLAT` reader - Bus latency"]
pub type BUSLAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUSLAT` writer - Bus latency"]
pub type BUSLAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SNTCFG0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn aset(&self) -> ASET_R {
        ASET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    pub fn ahld(&self) -> AHLD_R {
        AHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    pub fn dset(&self) -> DSET_R {
        DSET_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn buslat(&self) -> BUSLAT_R {
        BUSLAT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    #[must_use]
    pub fn aset(&mut self) -> ASET_W<0> {
        ASET_W::new(self)
    }
    #[doc = "Bits 4:7 - Address hold time"]
    #[inline(always)]
    #[must_use]
    pub fn ahld(&mut self) -> AHLD_W<4> {
        AHLD_W::new(self)
    }
    #[doc = "Bits 8:15 - Data setup time"]
    #[inline(always)]
    #[must_use]
    pub fn dset(&mut self) -> DSET_W<8> {
        DSET_W::new(self)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    #[must_use]
    pub fn buslat(&mut self) -> BUSLAT_W<16> {
        BUSLAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM/NOR flash timing configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sntcfg0](index.html) module"]
pub struct SNTCFG0_SPEC;
impl crate::RegisterSpec for SNTCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sntcfg0::R](R) reader structure"]
impl crate::Readable for SNTCFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sntcfg0::W](W) writer structure"]
impl crate::Writable for SNTCFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNTCFG0 to value 0x0fff_ffff"]
impl crate::Resettable for SNTCFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_ffff;
}
