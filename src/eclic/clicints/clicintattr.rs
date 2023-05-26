#[doc = "Register `CLICINTATTR` reader"]
pub struct R(crate::R<CLICINTATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLICINTATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLICINTATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLICINTATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLICINTATTR` writer"]
pub struct W(crate::W<CLICINTATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLICINTATTR_SPEC>;
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
impl From<crate::W<CLICINTATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLICINTATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHV` reader - SHV"]
pub type SHV_R = crate::BitReader<bool>;
#[doc = "Field `SHV` writer - SHV"]
pub type SHV_W<'a, const O: u8> = crate::BitWriter<'a, u8, CLICINTATTR_SPEC, bool, O>;
#[doc = "Field `TRIG` reader - TRIG"]
pub type TRIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIG` writer - TRIG"]
pub type TRIG_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CLICINTATTR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - SHV"]
    #[inline(always)]
    pub fn shv(&self) -> SHV_R {
        SHV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - TRIG"]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - SHV"]
    #[inline(always)]
    #[must_use]
    pub fn shv(&mut self) -> SHV_W<0> {
        SHV_W::new(self)
    }
    #[doc = "Bits 1:2 - TRIG"]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TRIG_W<1> {
        TRIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clicintattr Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clicintattr](index.html) module"]
pub struct CLICINTATTR_SPEC;
impl crate::RegisterSpec for CLICINTATTR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clicintattr::R](R) reader structure"]
impl crate::Readable for CLICINTATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clicintattr::W](W) writer structure"]
impl crate::Writable for CLICINTATTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLICINTATTR to value 0"]
impl crate::Resettable for CLICINTATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
