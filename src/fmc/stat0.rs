#[doc = "Register `STAT0` reader"]
pub struct R(crate::R<STAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT0` writer"]
pub struct W(crate::W<STAT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT0_SPEC>;
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
impl From<crate::W<STAT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSY` reader - The flash is busy bit"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `PGERR` reader - Program error flag bit"]
pub type PGERR_R = crate::BitReader<bool>;
#[doc = "Field `PGERR` writer - Program error flag bit"]
pub type PGERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `WPERR` reader - Erase/Program protection error flag bit"]
pub type WPERR_R = crate::BitReader<bool>;
#[doc = "Field `WPERR` writer - Erase/Program protection error flag bit"]
pub type WPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
#[doc = "Field `ENDF` reader - End of operation flag bit"]
pub type ENDF_R = crate::BitReader<bool>;
#[doc = "Field `ENDF` writer - End of operation flag bit"]
pub type ENDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The flash is busy bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Program error flag bit"]
    #[inline(always)]
    pub fn pgerr(&self) -> PGERR_R {
        PGERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Erase/Program protection error flag bit"]
    #[inline(always)]
    pub fn wperr(&self) -> WPERR_R {
        WPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of operation flag bit"]
    #[inline(always)]
    pub fn endf(&self) -> ENDF_R {
        ENDF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Program error flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn pgerr(&mut self) -> PGERR_W<2> {
        PGERR_W::new(self)
    }
    #[doc = "Bit 4 - Erase/Program protection error flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn wperr(&mut self) -> WPERR_W<4> {
        WPERR_W::new(self)
    }
    #[doc = "Bit 5 - End of operation flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn endf(&mut self) -> ENDF_W<5> {
        ENDF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat0](index.html) module"]
pub struct STAT0_SPEC;
impl crate::RegisterSpec for STAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat0::R](R) reader structure"]
impl crate::Readable for STAT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat0::W](W) writer structure"]
impl crate::Writable for STAT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT0 to value 0"]
impl crate::Resettable for STAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
