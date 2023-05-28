#[doc = "Register `FMPCFG` reader"]
pub struct R(crate::R<FMPCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMPCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMPCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMPCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMPCFG` writer"]
pub struct W(crate::W<FMPCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMPCFG_SPEC>;
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
impl From<crate::W<FMPCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMPCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMPEN` reader - Fast mode plus enable"]
pub type FMPEN_R = crate::BitReader<bool>;
#[doc = "Field `FMPEN` writer - Fast mode plus enable"]
pub type FMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, FMPCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Fast mode plus enable"]
    #[inline(always)]
    pub fn fmpen(&self) -> FMPEN_R {
        FMPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast mode plus enable"]
    #[inline(always)]
    #[must_use]
    pub fn fmpen(&mut self) -> FMPEN_W<0> {
        FMPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast mode plus configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpcfg](index.html) module"]
pub struct FMPCFG_SPEC;
impl crate::RegisterSpec for FMPCFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [fmpcfg::R](R) reader structure"]
impl crate::Readable for FMPCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmpcfg::W](W) writer structure"]
impl crate::Writable for FMPCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMPCFG to value 0"]
impl crate::Resettable for FMPCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
