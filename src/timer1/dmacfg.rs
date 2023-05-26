#[doc = "Register `DMACFG` reader"]
pub struct R(crate::R<DMACFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACFG` writer"]
pub struct W(crate::W<DMACFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACFG_SPEC>;
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
impl From<crate::W<DMACFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMATA` reader - DMA transfer access start address"]
pub type DMATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMATA` writer - DMA transfer access start address"]
pub type DMATA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DMACFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `DMATC` reader - DMA transfer count"]
pub type DMATC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMATC` writer - DMA transfer count"]
pub type DMATC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DMACFG_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - DMA transfer access start address"]
    #[inline(always)]
    pub fn dmata(&self) -> DMATA_R {
        DMATA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA transfer count"]
    #[inline(always)]
    pub fn dmatc(&self) -> DMATC_R {
        DMATC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA transfer access start address"]
    #[inline(always)]
    #[must_use]
    pub fn dmata(&mut self) -> DMATA_W<0> {
        DMATA_W::new(self)
    }
    #[doc = "Bits 8:12 - DMA transfer count"]
    #[inline(always)]
    #[must_use]
    pub fn dmatc(&mut self) -> DMATC_W<8> {
        DMATC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacfg](index.html) module"]
pub struct DMACFG_SPEC;
impl crate::RegisterSpec for DMACFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dmacfg::R](R) reader structure"]
impl crate::Readable for DMACFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmacfg::W](W) writer structure"]
impl crate::Writable for DMACFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACFG to value 0"]
impl crate::Resettable for DMACFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
